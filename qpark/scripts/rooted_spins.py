"""Verification of the ALL-SPINS-0 THEOREM (proved 2026-08-20).

Setup: (L, r) = a spanning tree of G x K2 with a root vertex; for each rung
u in the vertical support S, spin(u) = level of the endpoint of rung u
nearer to r in L.

THEOREM. (L, r) has all spins 0 if and only if L is polarized (level-0
graph a spanning tree of G, level-1 graph an S-rooted forest) and r lies at
level 0.

PROOF (two lines). The component graph B of L is a tree, bipartite between
level-0 and level-1 components, with one edge per rung. Root B at r's
component; spin(u) = level of the PARENT-side component of rung-edge u. All
spins 0 forces every parent to be a level-0 component; since B is bipartite
by level, no level-0 component can then have a parent (its parent would be
level-1), so the level-0 side is a single root component (H0 connected +
forest = spanning tree) and every level-1 component is a child with exactly
one rung, i.e. exactly one S-vertex: H1 is S-rooted. Conversely a polarized
lift rooted at level 0 visibly has all spins 0.  QED

CONSEQUENCE. Combined with spin-class equinumerosity within each (S, U)
class (Bernardi 2012, Thm 1 — proof constructive by induction; atomic move:
toggling the level of the single attaching edge of a pendant fiber), this
gives the per-level rooted bijection

  (spanning tree of G x K2, root)
     <->  (rooted spanning tree of G) x (root-2-colored S-rooted forest),

which is the set form of the support-product theorem with no factor of 1/2.

This script re-verifies the theorem exhaustively over all rooted spanning
trees of the prisms over nine small base graphs (108 384 rooted lifts).
"""
from collections import defaultdict, deque
from straightening_lab import base_graphs, prism_classes, polarized_of


def check_base(gname, nV, bedges):
    classes = prism_classes(nV, bedges)
    ok = bad = 0
    for (S, D, M), X in classes.items():
        P = polarized_of(nV, bedges, S, D, M, X)
        for x in X:
            adj = defaultdict(list)
            for lvl in (0, 1):
                for e in D:
                    a, b = bedges[e]
                    adj[2 * a + lvl].append(2 * b + lvl)
                    adj[2 * b + lvl].append(2 * a + lvl)
            for e in M:
                lvl = 1 if e in x else 0
                a, b = bedges[e]
                adj[2 * a + lvl].append(2 * b + lvl)
                adj[2 * b + lvl].append(2 * a + lvl)
            for u in S:
                adj[2 * u].append(2 * u + 1)
                adj[2 * u + 1].append(2 * u)
            for r in range(2 * nV):
                par = {r: None}
                dq = deque([r])
                while dq:
                    v = dq.popleft()
                    for w in adj[v]:
                        if w not in par:
                            par[w] = v
                            dq.append(w)
                allzero = all(par.get(2 * u + 1) == 2 * u for u in S)
                if allzero == ((x in P) and r % 2 == 0):
                    ok += 1
                else:
                    bad += 1
    return ok, bad


if __name__ == "__main__":
    total = mism = 0
    for gname, (nV, bedges) in base_graphs().items():
        ok, bad = check_base(gname, nV, bedges)
        total += ok + bad
        mism += bad
        print(f"{gname:<12} rooted lifts: {ok + bad:>7}  mismatches: {bad}")
    print(f"TOTAL {total} rooted lifts, {mism} mismatches")
