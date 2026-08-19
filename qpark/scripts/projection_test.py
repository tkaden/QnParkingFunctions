"""Does the support-product bijection factor through base projection?

For each spanning tree T of Q3 (direction d=0, base Q2 on {0,2,4,6}):
  S  = vertical support (base vertices whose vertical edge is in T)
  U  = multiset of base edges obtained by projecting the horizontal edges
       (levels 0 and 1) onto the base; multiplicities in {1,2}
Edge budget: |U| = 7 - |S| = (3 = tree edges of Q2) + (4 - |S| = edges of an
S-rooted forest of Q2).

Questions:
 Q1. Is every occurring U decomposable as (spanning tree of Q2) disjoint-
     union (S-rooted spanning forest of Q2), respecting multiplicities?
 Q2. Is the number of Q3-trees with data (S, U) proportional to
     2^(#multiplicity-1 edges of U) -- the free level choices -- times the
     number of decompositions? Tabulate N(S,U) / (#decomp) / 2^(#mult-1).
"""
import ast
from collections import Counter, defaultdict
from itertools import combinations

BASE = [0, 2, 4, 6]                       # base vertices (bit0 = 0)
BASE_EDGES = [(0, 2), (0, 4), (2, 6), (4, 6)]   # Q2 as 4-cycle


def base_of(v):
    return v & ~1


def spanning_trees_q2():
    # all 3-subsets of the 4 edges that connect: the 4-cycle minus one edge
    out = []
    for drop in range(4):
        out.append(frozenset(e for i, e in enumerate(BASE_EDGES) if i != drop))
    return out


def s_rooted_forests(S):
    """Edge subsets forming a forest where each component contains exactly
    one vertex of S (so #components = |S|, i.e. #edges = 4 - |S|)."""
    k = 4 - len(S)
    res = []
    for sub in combinations(BASE_EDGES, k):
        # union-find
        parent = {v: v for v in BASE}
        def find(x):
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x
        ok = True
        for a, b in sub:
            ra, rb = find(a), find(b)
            if ra == rb:
                ok = False
                break
            parent[ra] = rb
        if not ok:
            continue
        comps = defaultdict(set)
        for v in BASE:
            comps[find(v)].add(v)
        if all(len(c & set(S)) == 1 for c in comps.values()):
            res.append(Counter(sub))
    return res


TREES_Q2 = spanning_trees_q2()
FORESTS = {}  # S -> list of Counter(edge multiset)


def decomps(S, U):
    """Number of ways U = tree + S-rooted forest as multisets."""
    if S not in FORESTS:
        FORESTS[S] = s_rooted_forests(S)
    cnt = 0
    for t in TREES_Q2:
        rem = U - Counter(t)
        if sum(rem.values()) != 4 - len(S):
            continue
        if any(v > 1 for v in rem.values()):
            # forest part is a simple set of edges
            continue
        for f in FORESTS[S]:
            if f == rem:
                cnt += 1
    return cnt


data = defaultdict(int)
for line in open('/home/taden/github/QnParkingFunctions/qpark/results/q3_trees.txt'):
    par = ast.literal_eval(line.strip())
    S, U = [], Counter()
    for v in range(1, 8):
        p = par[v]
        if (v ^ p) == 1:                      # vertical edge (direction 0)
            S.append(base_of(v))
        else:                                 # horizontal: project to base
            e = tuple(sorted((base_of(v), base_of(p))))
            U[e] += 1
    data[(tuple(sorted(set(S))), tuple(sorted(U.items())))] += 1

print(f"{len(data)} distinct (S, U) classes over 384 trees")
bad_decomp = 0
ratios = Counter()
rows = []
for (S, U_items), ntrees in sorted(data.items()):
    U = Counter(dict(U_items))
    d = decomps(S, U)
    m1 = sum(1 for _, c in U.items() if c == 1)
    if d == 0:
        bad_decomp += 1
        rows.append((S, dict(U), ntrees, d, m1, None))
    else:
        ratio = ntrees / (d * 2 ** m1)
        ratios[ratio] += 1
        rows.append((S, dict(U), ntrees, d, m1, ratio))

print(f"classes with NO decomposition: {bad_decomp}")
print("distribution of N(S,U) / (#decomp * 2^mult1):", dict(ratios))
print()
print("sample rows (S | U | trees | #decomp | #mult-1 edges | ratio):")
for r in rows[:12]:
    print("  ", r)
# aggregate check per S: sum over U of #decomp * 2^m1 vs total trees with S
agg = defaultdict(lambda: [0, 0])
for (S, U_items), ntrees in data.items():
    U = Counter(dict(U_items))
    agg[S][0] += ntrees
    agg[S][1] += decomps(S, U) * 2 ** sum(1 for _, c in U.items() if c == 1)
print()
print("per-support totals: trees vs sum(#decomp * 2^mult1):")
for S, (a, b) in sorted(agg.items()):
    print(f"  S={S}: trees={a}  decomp-weighted={b}  ratio={a/b if b else float('inf'):.4f}")
