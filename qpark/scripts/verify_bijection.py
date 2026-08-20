"""End-to-end verification of the per-level bijection and the full hypercube
chain (the executable bijective proof).

Stage 1: per-level bijection on small bases, z = 1 and 2:
  encode: {z-colored rooted forests of G x K2} -> (A, B)
  checks: round-trip, injectivity, image = full product of factor sets,
  counts = det(zI+L) * det((z+2)I+L).
Stage 2: rooted spanning trees of Q3 (= prism over Q2): 3072 items.
Stage 3: the full chain Q3 -> color tuples in [2]^3 x [4]^3 x [6]: 3072
  distinct tuples, decode round-trip. That is a Prufer code for the cube.
"""
import itertools
import bijection as bj
from bijection import Forest, encode, decode, RUNG


# ---------- enumeration ----------
def prism_instance(nV, bedges):
    base = {("e", a, b): frozenset((a, b)) for a, b in bedges}
    return set(range(nV)), base


def enumerate_prism_forests(nV, bedges, z):
    """All z-colored rooted forests of G x K2 as Forest objects."""
    verts, base = prism_instance(nV, bedges)
    nodes = [2 * v + l for v in range(nV) for l in (0, 1)]
    nbrs = {n: [] for n in nodes}
    for eid, e in base.items():
        a, b = sorted(e)
        for l in (0, 1):
            nbrs[2 * a + l].append((2 * b + l, eid))
            nbrs[2 * b + l].append((2 * a + l, eid))
    for v in range(nV):
        nbrs[2 * v].append((2 * v + 1, RUNG))
        nbrs[2 * v + 1].append((2 * v, RUNG))
    choices = {n: [None] + nbrs[n] for n in nodes}
    out = []
    for combo in itertools.product(*(choices[n] for n in nodes)):
        parent = dict(zip(nodes, combo))
        # acyclic?
        ok = True
        state = {}
        for s in nodes:
            if s in state:
                continue
            path, m = [], s
            while m is not None and m not in state:
                state[m] = 1
                path.append(m)
                pe = parent[m]
                m = pe[0] if pe else None
            if m is not None and state.get(m) == 1:
                ok = False
                break
            for q in path:
                state[q] = 2
        if not ok:
            continue
        # edge copies: no eid twice at same level (parent map cannot: each
        # child uses its own arc; same eid at same level = two children of
        # one arc copy? an arc copy is (edge, level); two nodes could both
        # claim it only as (a->b) and (b->a) which is a 2-cycle, excluded.)
        roots = [n for n in nodes if parent[n] is None]
        for cols in itertools.product(range(z), repeat=len(roots)):
            color = dict(zip(roots, cols))
            yield Forest(verts, base, parent, color)


def enumerate_base_forests(nV, bedges, z):
    """All z-colored rooted forests of G itself, as canonical triples."""
    base = {("e", a, b): frozenset((a, b)) for a, b in bedges}
    nbrs = {v: [] for v in range(nV)}
    for eid, e in base.items():
        a, b = sorted(e)
        nbrs[a].append((b, eid))
        nbrs[b].append((a, eid))
    out = set()
    for combo in itertools.product(*([(None, None)] + nbrs[v] for v in range(nV))):
        parent = {v: combo[v] for v in range(nV)}
        state, ok = {}, True
        for s in range(nV):
            if s in state:
                continue
            path, m = [], s
            while m is not None and m not in state:
                state[m] = 1
                path.append(m)
                m = parent[m][0] if parent[m][0] is not None else None
            if m is not None and state.get(m) == 1:
                ok = False
                break
            for q in path:
                state[q] = 2
        if not ok:
            continue
        roots = [v for v in range(nV) if parent[v][0] is None]
        for cols in itertools.product(range(z), repeat=len(roots)):
            col = dict(zip(roots, cols))
            out.add(triple_key(({v: parent[v][0] for v in range(nV)},
                                {v: parent[v][1] for v in range(nV)}, col)))
    return out


def triple_key(t):
    par, eid, col = t
    return (frozenset(par.items()), frozenset((k, v) for k, v in eid.items() if v),
            frozenset(col.items()))


# ---------- stage 1 ----------
def stage1():
    bases = {
        "edge": (2, [(0, 1)]),
        "path3": (3, [(0, 1), (1, 2)]),
        "triangle": (3, [(0, 1), (1, 2), (0, 2)]),
    }
    for gname, (nV, bedges) in bases.items():
        verts, base = prism_instance(nV, bedges)
        for z in (1, 2):
            seen = {}
            n_forests = 0
            for F in enumerate_prism_forests(nV, bedges, z):
                n_forests += 1
                A, B = encode(F, z)
                key = (triple_key(A), triple_key(B))
                assert key not in seen, f"{gname} z={z}: encode not injective"
                seen[key] = True
                G2 = decode(A, B, z, verts, base)
                assert G2.key() == F.key(), f"{gname} z={z}: round-trip failed"
            SA = enumerate_base_forests(nV, bedges, z)
            SB = enumerate_base_forests(nV, bedges, z + 2)
            assert n_forests == len(SA) * len(SB), \
                f"{gname} z={z}: count {n_forests} != {len(SA)}*{len(SB)}"
            got_A = {k[0] for k in seen}
            got_B = {k[1] for k in seen}
            assert got_A <= SA and got_B <= SB
            assert len(seen) == n_forests
            # surjectivity: image is the full product
            prod = {(a, b) for a in SA for b in SB}
            assert set(seen) == prod, f"{gname} z={z}: image != product"
            print(f"stage1 {gname:<9} z={z}: {n_forests:>7} forests "
                  f"<-> {len(SA)} x {len(SB)}  [bijection verified]")


# ---------- stage 2/3: the hypercube chain ----------
def q_edges(k):
    n = 1 << k
    return [(v, v ^ (1 << b)) for v in range(n) for b in range(k) if v < v ^ (1 << b)]


def qeid(k, a, b):
    return ("e", min(a, b), max(a, b))


def triple_to_prism_forest(k, t, ):
    """View a forest-triple on Q_k vertices as a prism Forest over Q_{k-1}."""
    par, eidm, col = t
    half = 1 << (k - 1)
    verts = set(range(half))
    base = {qeid(k - 1, a, b): frozenset((a, b)) for a, b in q_edges(k - 1)}
    parent, color = {}, {}
    for w in range(1 << k):
        v, l = w & (half - 1), w >> (k - 1)
        n = 2 * v + l
        pw = par[w]
        if pw is None:
            parent[n] = None
            color[n] = col[w]
        elif pw == w ^ half:
            parent[n] = (bj.twin(n), RUNG)
        else:
            v2 = pw & (half - 1)
            parent[n] = (2 * v2 + l, qeid(k - 1, v, v2))
    return Forest(verts, base, parent, color), verts, base


def factor_to_triple(k, A):
    """A factor over Q_{k} vertices (already in triple form) — identity."""
    return A


def chain_encode(k, z, t):
    """Forest-triple on Q_k -> nested tuple of terminal colors."""
    if k == 0:
        return (t[2][0],)
    F, verts, base = triple_to_prism_forest(k, t)
    A, B = encode(F, z)
    return chain_encode(k - 1, z, A) + chain_encode(k - 1, z + 2, B)


def tree_chain_encode(k, t):
    """Rooted spanning tree of Q_k (root color 0) -> flat color tuple in
    the exact box  prod over k' of [2k']^C(k, k')  (Prufer code)."""
    if k == 0:
        return ()
    F, verts, base = triple_to_prism_forest(k, t)
    A, B = encode(F, 1)          # tree: z = 1, root color 0
    # A is again a rooted tree with color 0; B uses colors {1, 2}: shift to
    # {0, 1} and continue as an honest 2-colored forest chain
    B2 = (B[0], B[1], {v: c - 1 for v, c in B[2].items()})
    return tree_chain_encode(k - 1, A) + chain_encode(k - 1, 2, B2)


def tree_chain_decode(k, colors):
    if k == 0:
        return ({0: None}, {0: None}, {0: 0}), colors
    tA, rest = tree_chain_decode(k - 1, colors)
    tB, rest = chain_decode(k - 1, 2, rest)
    tB = (tB[0], tB[1], {v: c + 1 for v, c in tB[2].items()})
    half = 1 << (k - 1)
    verts = set(range(half))
    base = {qeid(k - 1, a, b): frozenset((a, b)) for a, b in q_edges(k - 1)}
    F = decode(tA, tB, 1, verts, base)
    par, eidm, col = {}, {}, {}
    for w in range(1 << k):
        v, l = w & (half - 1), w >> (k - 1)
        n = 2 * v + l
        pe = F.parent[n]
        if pe is None:
            par[w], eidm[w] = None, None
            col[w] = F.color[n]
        elif pe[1] == RUNG:
            par[w], eidm[w] = w ^ half, None
        else:
            v2 = pe[0] // 2
            par[w] = v2 + (l << (k - 1))
            eidm[w] = qeid(k, w, par[w])
    return (par, eidm, col), rest


def chain_decode(k, z, colors):
    """Inverse: consume the flat color tuple, rebuild the Q_k forest-triple."""
    if k == 0:
        return ({0: None}, {0: None}, {0: colors[0]}), colors[1:]
    tA, rest = chain_decode(k - 1, z, colors)
    tB, rest = chain_decode(k - 1, z + 2, rest)
    half = 1 << (k - 1)
    verts = set(range(half))
    base = {qeid(k - 1, a, b): frozenset((a, b)) for a, b in q_edges(k - 1)}
    F = decode(tA, tB, z, verts, base)
    # prism Forest over Q_{k-1} -> triple on Q_k
    par, eidm, col = {}, {}, {}
    for w in range(1 << k):
        v, l = w & (half - 1), w >> (k - 1)
        n = 2 * v + l
        pe = F.parent[n]
        if pe is None:
            par[w], eidm[w] = None, None
            col[w] = F.color[n]
        elif pe[1] == RUNG:
            par[w], eidm[w] = w ^ half, None
        else:
            v2 = pe[0] // 2
            par[w] = v2 + (l << (k - 1))
            eidm[w] = qeid(k, w, par[w])
    return (par, eidm, col), rest


def rooted_trees_q(k):
    """All (spanning tree, root) of Q_k as forest-triples with root color 0."""
    import subprocess, ast, os
    n = 1 << k
    # enumerate spanning trees by brute force over parent maps (k <= 3)
    edges = q_edges(k)
    nbrs = {v: [] for v in range(n)}
    for a, b in edges:
        nbrs[a].append(b)
        nbrs[b].append(a)
    out = []
    for r in range(n):
        # all parent maps rooted at r
        others = [v for v in range(n) if v != r]
        for combo in itertools.product(*(nbrs[v] for v in others)):
            par = {r: None}
            for v, p in zip(others, combo):
                par[v] = p
            # connectivity/acyclicity toward r
            ok = True
            for s in range(n):
                seen, m = set(), s
                while m is not None and m != r and m not in seen:
                    seen.add(m)
                    m = par[m]
                if m is None or (m != r):
                    ok = False
                    break
            if not ok:
                continue
            eidm = {v: (None if par[v] is None else qeid(k, v, par[v]))
                    for v in range(n)}
            out.append((dict(par), eidm, {r: 0}))
    return out


def stage23():
    bj.DEBUG = False
    trees = rooted_trees_q(3)
    print(f"stage2: rooted spanning trees of Q3 enumerated: {len(trees)} "
          f"(expected 8*384 = 3072)")
    assert len(trees) == 3072
    codes = {}
    for t in trees:
        code = tree_chain_encode(3, t)
        assert code not in codes, "chain not injective"
        codes[code] = t
        t2, rest = tree_chain_decode(3, code)
        assert not rest
        assert t2[0] == t[0] and t2[2] == t[2], "chain round-trip failed"
    print(f"stage3: {len(codes)} distinct color tuples; all round-trips OK")
    doms = [max(c[i] for c in codes) + 1 for i in range(len(next(iter(codes))))]
    prod = 1
    for d in doms:
        prod *= d
    print(f"stage3: tuple slot domains = {doms}, product = {prod}")
    assert prod == len(codes), "codes do not fill the exact box"
    # surjectivity onto the box: 3072 distinct tuples in a 3072 box = onto
    print("stage3: the code is a bijection ONTO the box "
          "[2]^3 x [4]^3 x [6] — a Prufer code for the cube")


if __name__ == "__main__":
    stage1()
    stage23()
    print("ALL VERIFICATIONS PASSED")
