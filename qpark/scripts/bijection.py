"""The per-level bijection, executable.

Objects: COLORED ROOTED FORESTS of the prism G x K2 over a base instance.
Nodes are (v, level) encoded 2v+l. The forest is a parent map
    parent[node] = None (root)  or  (parent_node, edge_id)
where edge_id is "rung" for vertical arcs and a base-edge id for horizontal
arcs (child and parent at the same level). Roots carry colors.

The per-level bijection

  { z-colored rooted spanning forests of G x K2 }
     <-> { z-colored rooted forests of G } x { (z+2)-colored rooted forests of G }

is realized by (1) spin normalization -- Bernardi's Theorem-1 induction made
explicit: `flip_spin` is an involution on each projection class reversing
one rung's orientation and nothing else; (2) the all-spins-zero star normal
form: level 0 reads off as the z-colored factor, level-1 pieces as the
(z+2)-colored factor (own-rooted pieces keep old colors < z; rung-attached
pieces take color z + recorded spin).

Rooted spanning TREES are the 1-component case: their level-1 factor uses
only the two new colors.
"""
from collections import defaultdict

DEBUG = True
RUNG = "rung"


def base_of(n):
    return n // 2


def lvl(n):
    return n % 2


def twin(n):
    return n ^ 1


class Forest:
    def __init__(self, verts, base, parent, color):
        self.verts = set(verts)          # active base vertices
        self.base = dict(base)           # edge_id -> frozenset({a, b})
        self.parent = dict(parent)       # node -> None | (node, eid)
        self.color = dict(color)         # root node -> color
        if DEBUG:
            self.validate()

    def clone(self):
        return Forest(self.verts, self.base, self.parent, self.color)

    def validate(self):
        ns = {2 * v + l for v in self.verts for l in (0, 1)}
        assert set(self.parent) == ns
        eid_use = defaultdict(list)
        for n, pe in self.parent.items():
            if pe is None:
                assert n in self.color, f"uncolored root {n}"
                continue
            p, eid = pe
            assert p in self.parent
            if eid == RUNG:
                assert p == twin(n)
            else:
                assert lvl(p) == lvl(n)
                assert self.base.get(eid) == frozenset((base_of(n), base_of(p))), \
                    f"arc {n}->{p} vs edge {eid}={self.base.get(eid)}"
                eid_use[eid].append(lvl(n))
        for eid, lvls in eid_use.items():
            assert len(lvls) <= 2 and len(set(lvls)) == len(lvls), \
                f"edge {eid} used twice at one level"
        for n in self.color:
            assert self.parent[n] is None
        # acyclicity
        state = {}
        for start in self.parent:
            if start in state:
                continue
            path = []
            m = start
            while m is not None and m not in state:
                state[m] = "gray"
                path.append(m)
                m = self.parent[m][0] if self.parent[m] else None
            assert m is None or state.get(m) == "black", f"cycle via {start}"
            for q in path:
                state[q] = "black"

    # ---- structure ----
    def S(self):
        return {base_of(n) for n, pe in self.parent.items()
                if pe is not None and pe[1] == RUNG}

    def spin(self, u):
        if self.parent.get(2 * u + 1) == (2 * u, RUNG):
            return 0
        if self.parent.get(2 * u) == (2 * u + 1, RUNG):
            return 1
        raise ValueError(f"no rung at {u}")

    def indeg_fiber(self, u):
        return sum(1 for n, pe in self.parent.items()
                   if pe is not None and pe[1] != RUNG and base_of(pe[0]) == u)

    def hout(self, n):
        pe = self.parent.get(n)
        return pe if (pe is not None and pe[1] != RUNG) else None

    def key(self):
        return (frozenset((n, pe if pe is None else (pe[0], pe[1]))
                          for n, pe in self.parent.items()),
                frozenset(self.color.items()))


# ---------------- spin flip ----------------
def flip_spin(F, u_star):
    F = F.clone()
    # tag root colors with unique serials so transported roots can be located
    # unambiguously during restores, then strip the tags at the end
    tags = {}
    for i, (n, c) in enumerate(sorted(F.color.items())):
        F.color[n] = ("tag", i, c)
        tags[("tag", i, c)] = c
    _flip(F, u_star)
    F.color = {n: tags[c] if isinstance(c, tuple) and c and c[0] == "tag" else c
               for n, c in F.color.items()}
    if DEBUG:
        F.validate()
    return F


def _reducible(F, u_star):
    S = F.S()
    assert u_star in S
    if F.indeg_fiber(u_star) == 0:
        return ("flip", u_star)
    for u in sorted(F.verts):
        if u == u_star:
            continue
        if u in S and F.indeg_fiber(u) == 0:
            return ("a", u)
        if u not in S and F.indeg_fiber(u) <= 1:
            return ("b", u)
    raise AssertionError("no reducible vertex — counting argument violated")


def _flip(F, u_star):
    kind, u = _reducible(F, u_star)
    if kind == "flip":
        _pendant_flip(F, u)
    elif kind == "a":
        rec = _reduce_a(F, u)
        _flip(F, u_star)
        _restore_a(F, rec)
    else:
        rec = _reduce_b(F, u)
        _flip(F, u_star)
        _restore_b(F, rec)


def _fiber_top(F, u):
    n0, n1 = 2 * u, 2 * u + 1
    if F.parent[n1] == (n0, RUNG):
        return n0, n1
    assert F.parent[n0] == (n1, RUNG)
    return n1, n0


def _pendant_flip(F, u):
    top, bot = _fiber_top(F, u)
    out = F.hout(top)
    if out is None:
        c = F.color.pop(top)
        F.color[bot] = c
        F.parent[bot] = None
    else:
        p, eid = out
        F.parent[bot] = (twin(p), eid)
    F.parent[top] = (bot, RUNG)


def _reduce_a(F, u):
    top, bot = _fiber_top(F, u)
    out = F.hout(top)
    rec = {"u": u, "spin": lvl(top),
           "root_color": F.color.get(top) if out is None else None,
           "out": None if out is None else (base_of(out[0]), out[1])}
    for n in (2 * u, 2 * u + 1):
        F.parent.pop(n)
        F.color.pop(n, None)
    F.verts.discard(u)
    rec["edges"] = {eid: e for eid, e in F.base.items() if u in e}
    for eid in rec["edges"]:
        F.base.pop(eid)
    if DEBUG:
        F.validate()
    return rec


def _restore_a(F, rec):
    u, sp = rec["u"], rec["spin"]
    F.verts.add(u)
    F.base.update(rec["edges"])
    top, bot = 2 * u + sp, 2 * u + (1 - sp)
    F.parent[bot] = (top, RUNG)
    if rec["out"] is None:
        F.parent[top] = None
        F.color[top] = rec["root_color"]
    else:
        w, eid = rec["out"]
        F.parent[top] = (2 * w + sp, eid)
    if DEBUG:
        F.validate()


def _reduce_b(F, u):
    n0, n1 = 2 * u, 2 * u + 1
    children = [n for n, pe in F.parent.items()
                if pe is not None and pe[1] != RUNG and pe[0] in (n0, n1)]
    assert len(children) <= 1
    rec = {"u": u, "sides": {}, "child": None}
    for n in (n0, n1):
        pe = F.parent[n]
        if pe is None:
            rec["sides"][lvl(n)] = ("root", F.color[n])
        else:
            assert pe[1] != RUNG
            rec["sides"][lvl(n)] = ("out", base_of(pe[0]), pe[1])
    if children:
        ch = children[0]
        l0 = lvl(ch)
        ch_eid = F.parent[ch][1]
        rec["child"] = (base_of(ch), l0, ch_eid)
        side = rec["sides"][l0]
        if side[0] == "out":
            _, w, out_eid = side
            meid = ("m", u, ch_eid, out_eid)
            F.base[meid] = frozenset((base_of(ch), w))
            F.parent[ch] = (2 * w + l0, meid)
            rec["merged"] = meid
        else:
            F.parent[ch] = None
            F.color[ch] = side[1]
            rec["transport"] = ch
            rec["transport_tag"] = side[1]
    for n in (n0, n1):
        F.parent.pop(n)
        F.color.pop(n, None)
    F.verts.discard(u)
    rec["edges"] = {eid: e for eid, e in F.base.items()
                    if u in e and eid != rec.get("merged")}
    for eid in rec["edges"]:
        F.base.pop(eid)
    if DEBUG:
        F.validate()
    return rec


def _restore_b(F, rec):
    u = rec["u"]
    F.verts.add(u)
    F.base.update(rec["edges"])
    if rec["child"] is None:
        for l in (0, 1):
            _restore_side(F, u, l, rec["sides"][l])
    else:
        cb, l0, ch_eid = rec["child"]
        side = rec["sides"][l0]
        if side[0] == "out":
            meid = rec["merged"]
            hit = [(n, pe) for n, pe in F.parent.items()
                   if pe is not None and pe[1] == meid]
            assert len(hit) == 1, "merged arc not found uniquely"
            n, (p, _) = hit[0]
            lcur = lvl(n)
            F.base.pop(meid)
            F.parent[n] = (2 * u + lcur, ch_eid)
            F.parent[2 * u + lcur] = (p, side[2])
            _restore_side(F, u, 1 - lcur, rec["sides"][1 - l0])
        else:
            ch = rec["transport"]
            tag = rec["transport_tag"]
            cand = [n for n in (ch, twin(ch))
                    if F.parent.get(n) is None and F.color.get(n) == tag]
            assert len(cand) == 1, "transported root not located uniquely"
            n = cand[0]
            lcur = lvl(n)
            c = F.color.pop(n)
            F.parent[n] = (2 * u + lcur, ch_eid)
            F.parent[2 * u + lcur] = None
            F.color[2 * u + lcur] = c
            _restore_side(F, u, 1 - lcur, rec["sides"][1 - l0])
    if DEBUG:
        F.validate()


def _restore_side(F, u, l, side):
    n = 2 * u + l
    if side[0] == "root":
        F.parent[n] = None
        F.color[n] = side[1]
    else:
        _, w, eid = side
        F.parent[n] = (2 * w + l, eid)


# ---------------- normalization and factor readout ----------------
def normalize(F):
    F = F.clone()
    sigma = {}
    for u in sorted(F.S()):
        sigma[u] = F.spin(u)
        if sigma[u] == 1:
            F = flip_spin(F, u)
    return F, sigma


def denormalize(F0, sigma):
    F = F0.clone()
    for u in sorted(sigma, reverse=True):
        if sigma[u] == 1:
            F = flip_spin(F, u)
    return F


def read_normal_form(F0, sigma, z):
    """-> (A, B): each is (parent: v->v'|None, eid: v->edge id|None, color: v->c)."""
    A = ({}, {}, {})
    B = ({}, {}, {})
    for v in sorted(F0.verts):
        n0, n1 = 2 * v, 2 * v + 1
        pe0 = F0.parent[n0]
        assert pe0 is None or pe0[1] != RUNG, "spin-1 rung in normal form"
        if pe0 is None:
            A[0][v], A[1][v] = None, None
            A[2][v] = F0.color[n0]
        else:
            A[0][v], A[1][v] = base_of(pe0[0]), pe0[1]
        pe1 = F0.parent[n1]
        if pe1 is not None and pe1[1] == RUNG:
            B[0][v], B[1][v] = None, None
            B[2][v] = z + sigma[v]
        elif pe1 is None:
            B[0][v], B[1][v] = None, None
            B[2][v] = F0.color[n1]
        else:
            B[0][v], B[1][v] = base_of(pe1[0]), pe1[1]
    return A, B


def build_normal_form(A, B, z, verts, base):
    parent, color, sigma = {}, {}, {}
    for v in verts:
        n0, n1 = 2 * v, 2 * v + 1
        if A[0][v] is None:
            parent[n0] = None
            color[n0] = A[2][v]
        else:
            parent[n0] = (2 * A[0][v], A[1][v])
        if B[0][v] is None:
            c = B[2][v]
            if c >= z:
                parent[n1] = (n0, RUNG)
                sigma[v] = c - z
            else:
                parent[n1] = None
                color[n1] = c
        else:
            parent[n1] = (2 * B[0][v] + 1, B[1][v])
    return Forest(verts, base, parent, color), sigma


def encode(F, z):
    F0, sigma = normalize(F)
    return read_normal_form(F0, sigma, z)


def decode(A, B, z, verts, base):
    F0, sigma = build_normal_form(A, B, z, verts, base)
    return denormalize(F0, sigma)
