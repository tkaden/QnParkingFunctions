"""Candidate v4 for the straightening map.

Claim to test: for every valid lift x of class (S, U), there is a UNIQUE
connected sub-collection C of the component-tree B(x) such that the SET
union T of the edge supports of C's components is a spanning tree of G
containing D, and F := U - T (multiset) is an S-rooted forest. Then the map
x -> (T, F) should have fibers of size exactly 2^(|S|-1).
"""
from collections import defaultdict
from straightening_lab import (
    base_graphs, prism_classes, is_forest, comps_of, polarized_of,
)


def lift_data(nV, bedges, S, D, M, x):
    comps = []
    for lvl in (0, 1):
        es = [e for e in D] + [e for e in M if (e in x) == (lvl == 1)]
        p = list(range(nV))
        def find(z):
            while p[z] != z:
                p[z] = p[p[z]]
                z = p[z]
            return z
        for e in es:
            a, b = bedges[e]
            ra, rb = find(a), find(b)
            if ra != rb:
                p[ra] = rb
        groups = defaultdict(lambda: [set(), set()])
        for v in range(nV):
            groups[find(v)][0].add(v)
        for e in es:
            groups[find(bedges[e][0])][1].add(e)
        for g in groups.values():
            comps.append((lvl, frozenset(g[0]), frozenset(g[1])))
    badj = defaultdict(set)
    for u in S:
        c0 = next(i for i, (l, vs, _) in enumerate(comps) if l == 0 and u in vs)
        c1 = next(i for i, (l, vs, _) in enumerate(comps) if l == 1 and u in vs)
        badj[c0].add(c1)
        badj[c1].add(c0)
    return comps, badj


def connected_in_b(nodes, badj):
    nodes = set(nodes)
    start = next(iter(nodes))
    seen = {start}
    stack = [start]
    while stack:
        c = stack.pop()
        for d2 in badj[c]:
            if d2 in nodes and d2 not in seen:
                seen.add(d2)
                stack.append(d2)
    return seen == nodes


def v4_extract(nV, bedges, S, D, M, x):
    """All (T, F) arising from connected B-subtrees via set-union of supports."""
    comps, badj = lift_data(nV, bedges, S, D, M, x)
    n = len(comps)
    found = set()
    for mask in range(1, 1 << n):
        nodes = [i for i in range(n) if mask >> i & 1]
        if not connected_in_b(nodes, badj):
            continue
        T = set()
        for i in nodes:
            T |= comps[i][2]
        if len(T) != nV - 1 or not D <= T:
            continue
        Te = [bedges[e] for e in T]
        if not (is_forest(nV, Te) and len(comps_of(nV, Te)) == 1):
            continue
        F = set(D) | (set(M) - T)
        Fe = [bedges[e] for e in F]
        if not is_forest(nV, Fe):
            continue
        cs = comps_of(nV, Fe)
        if len(cs) != len(S) or any(len(c & S) != 1 for c in cs):
            continue
        found.add((frozenset(T), frozenset(F)))
    return found


def run():
    grand = defaultdict(int)
    print(f"{'base':<12}{'classes':>8}{'unique(T,F)':>13}{'fibers-ok':>11}")
    for gname, (nV, bedges) in base_graphs().items():
        classes = prism_classes(nV, bedges)
        tot = uniq = fib = 0
        for (S, D, M), X in classes.items():
            tot += 1
            want = 2 ** (len(S) - 1)
            fibers = defaultdict(int)
            all_u = True
            for x in X:
                tf = v4_extract(nV, bedges, S, D, set(M), x)
                grand[len(tf)] += 1
                if len(tf) != 1:
                    all_u = False
                else:
                    fibers[next(iter(tf))] += 1
            if all_u:
                uniq += 1
                if all(v == want for v in fibers.values()):
                    fib += 1
        print(f"{gname:<12}{tot:>8}{uniq:>10}/{tot}{fib:>8}/{tot}")
    print("(T,F)-count histogram over all valid lifts:", dict(sorted(grand.items())))


if __name__ == "__main__":
    run()
