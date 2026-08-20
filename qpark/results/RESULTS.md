# Experimental results — 2026-08-19

Phase 1 (Rust rewrite) and the first round of Phase 2 experiments.
All computations by `qpark` unless noted; independent Python re-implementations
were used to cross-check the SNF and the transversal searches.

## Counts (all verified)

| n | genus g | maximal PFs = T(1,0) | spanning trees | time |
|---|---------|----------------------|----------------|------|
| 2 | 1       | 3                    | 4              | —    |
| 3 | 5       | 133                  | 384            | ms   |
| 4 | 17      | **3 040 575**        | **42 467 328** | 1.5 s|

Q4 matches the linear coefficient of Wilf's chromatic polynomial of Q4, the
Matrix-Tree determinant, and Stanley's formula — the first reproduction of the
3 040 575 count by direct enumeration that we know of. Q5 (~8.2 × 10^16) is not
listable; its count comes from the chromatic polynomial of Q5 (OEIS A334278).

## New data: T_{Q4}(1, y) (h-vector of the cographic matroid of Q4)

Parking functions of Q4 by degree 0..17 (see `q4_hvector.txt`):

1, 15, 120, 680, 3044, 11388, 36808, 104984, 267894, 616906, 1287688,
2436504, 4156516, 6306364, 8278008, 8903016, 7016817, 3040575.

Total 42 467 328 (trees), top coefficient 3 040 575 (maximal PFs = T(1,0)).
The full Tutte polynomial of Q4 appears to be unpublished; this diagonal is a
candidate OEIS submission.

## Sandpile groups (Smith normal forms of the reduced Laplacian)

- K(Q3) = Z2 × Z8 × Z24 (order 384, 2-rank 3)
- K(Q4) = Z2 × Z2 × Z8 × Z24 × Z24 × Z24 × Z96 (order 42 467 328, 2-rank 7)

Invariant-factor counts (3 and 7) match Bai's theorem (2^{n−1} − 1). All
maximal parking functions map to pairwise distinct group classes (verified at
the full 3 M scale for Q4) — consistent with superstables being coset
representatives.

## Q5: what is and isn't reachable

Direct enumeration is out by ~7 orders of magnitude (8.2 × 10^16 leaves at the
measured ~2M leaves/s ≈ 1,300 years). Two things are exact or statistically
checkable:

- **K(Q5) = Z2^5 × Z6 × Z24^4 × Z48 × Z192^3 × Z960** (SNF of the 31×31
  reduced Laplacian, < 1 ms; `q5_snf.txt`). Consistency with Bai (2003):
  order = c(Q5) exactly; 15 invariant factors (Thm 1.1: 2^{n−1} − 1); six Z2's
  in elementary-divisor form (Thm 1.3: a_5 = 2^3 − 2^1 = 6); odd part
  Z3^10 × Z5 (Thm 1.2). The full Sylow-2 is Z2^6 × Z8^4 × Z16 × Z64^4 — the
  2-part is open in general (Gao–Marx-Kuo–McDonald–Yuen 2024 give only the
  largest n−1 factors), so this explicit n = 5 value is a useful data point.
- **Knuth random-descent estimator** (`qpark estimate`): unbiased leaf-count
  and box-volume-total estimates from random root-to-leaf walks. Validated on
  Q3/Q4 (within ~1.5 standard errors of truth). Q5 at 200M samples:
  maximal-PF estimate 7.37e16 ± 0.50e16 vs the chromatic-polynomial value
  8.1769e16 (−1.6σ, consistent — the first check on that number outside
  chromatic-polynomial computations). CAVEAT: the tree-side estimate came out
  17σ below the exactly-known tree count, exposing the estimator's heavy-tail
  undercoverage; the leaf estimate is order-of-magnitude reliable but likely
  also biased somewhat low. Publication-grade verification would need
  importance sampling.

## The spin experiment (Direction 1) — a sharp positive/negative dichotomy

Built: BCT Theorem 4.6 bijection pi/mu (validated: mu(pi(f)) = f for every
parking function of Q2/Q3 and 12 random simple graphs; pi onto all spanning
trees), Wilson's algorithm for exact uniform spanning trees, and a spin
statistic tester (`qpark spins`).

Candidate 0 (out-edge spins at weight-≥2 vertices): DEAD — the spin of a
vertex's out-edge is a deterministic function of its direction (parentᵢ =
1 − vᵢ), carrying no free bits. Killed by inspection of the Q3 table.

Candidate 1 (Bernardi vertical-edge spins conditioned on vertical support,
transported through pi): for trees ROOTED AT 0, decisively NON-uniform
(Q3 exhaustive: chi2 = 330.7 on 65 dof per direction; Q4 2M samples:
chi2 ≈ 1.39e6 on 6305 dof). But for the SAME trees with the root freed
(uniform over all (tree, root) pairs): Q3 exhaustive — **exactly uniform,
chi2 = 0.0, every spin-vector count identical in all 45 classes**; Q4 (2M
samples) and Q5 (1M samples) — chi2 within ~1σ of dof in every direction.

Interpretation: Bernardi's free Z2 bits are real and constructive, but they
live exactly in the rooted-anywhere world; FIXING THE ROOT is precisely what
breaks the product structure. Consistently, the clean formula is for rooted
trees: 2^n · c(Q_n) = prod_{v≠0} (2·wgt(v)) — one (coordinate-in-support,
bit) pair per non-root vertex. Bijective program, reformulated: build the
bijection {rooted spanning trees} ↔ prod_{v≠0} (supp(v) × {0,1}) using
Bernardi's rerouting involutions for the bit part, and only then quotient by
the 2^n root choices to reach parking functions rooted at q. Files:
`q3_spins.txt`, `q4_spins_sampled.txt`, `q5_spins_sampled.txt`.

## THE EXECUTABLE BIJECTIVE PROOF (2026-08-20, continuation session)

Implemented the complete chain (scripts/bijection.py + verify_bijection.py):
spin flips per Bernardi's induction (pendant-fiber flip; splice-and-recurse
with unique edge-ids for merged arcs and tagged root colors for transported
roots), the colored star normal form, and the factor readout. VERIFIED:

- Per-level bijection F_z(G x K2) <-> F_z(G) x F_{z+2}(G): EXHAUSTIVE and
  EXACT (injective, onto the full product, round-trip) for 6 (base, z)
  combinations up to 9,800 forests each.
- n=3: ALL 3,072 rooted spanning trees of Q3 <-> the exact box
  [2]^3 x [4]^3 x [6], perfect round-trips. A Prufer code for the cube.
- n=4: 200 uniform random rooted trees: distinct codes in the box
  [2]^4 [4]^6 [6]^4 [8] (size 679,477,248), all round-trips OK.

Paper now states Theorem "colored hierarchy" and Theorem "Prufer code for
the hypercube": an explicit algorithmic bijection rooted-trees(Qn) <->
prod_k [2k]^C(n,k), i.e. A BIJECTIVE PROOF OF STANLEY'S FORMULA, with the
honest qualifier that the spin normalization is recursive (explicit,
terminating, machine-verified) rather than closed-form; making it
closed-form is the one remaining refinement (paper Problem 1). Requires
human review (Tyler, Brian; ideally Bernardi) before public claims.

## THE ROOTED NORMAL FORM (proved 2026-08-20, "close it out" session)

Root the lift anywhere; spin(u) = level of rung u's endpoint nearer the
root. NEW THEOREM (two-line bipartite proof, verified over 108,384 rooted
prism trees of 9 base graphs, zero mismatches — scripts/rooted_spins.py):

    all spins 0  <=>  the lift is POLARIZED and the root is at level 0.

(B is bipartite by level; "every parent at level 0" collapses it to a star.)
Combined with spin-class equinumerosity per (S,U) class — Bernardi 2012
Thm 1, whose inductive proof is constructive (atomic move: toggle the level
of a pendant fiber's single attaching M-edge) — this makes the per-level
ROOTED bijection a THEOREM:

  (tree of G x K2, root) <-> (rooted base tree) x (spin-colored S-rooted forest)

i.e. eq:setform holds by explicit correspondence, class by class, with no
factor 1/2 (paper Thm "rooted straightening").

What genuinely remains for a fully self-contained bijective proof of
Stanley's formula (both stated precisely in the paper):
 1. a CLOSED-FORM spin normalization (Bernardi's is recursive);
 2. the COLORED HIERARCHY: z-colored rooted forests of G x K2 <->
    (z-colored) x ((z+2)-colored) rooted forests of G, bijectively, for all
    even z (the +2 = the two spin values; algebraic proof = the same block
    identity det(zI + L_{GxK2}) = det(zI+L_G) det((z+2)I+L_G)); iterating
    over the n directions then yields Stanley's formula via Pascal.

Failed candidates recorded for posterity (scripts/straightening_lab.py,
candidate_v4.py): nearest-polarized Hamming matching (3035/3042 classes,
fails on K4); mirror-move orbits (wrong equivalence — |S|=1 classes prove
fibers are not move-orbits); whole-component splits (1608 lifts have none);
B-subtree support-unions (666 lifts have none, 1864 ambiguous). Every
failure narrowed the search: fibers-when-unique were ALWAYS perfect, which
pointed at the rooted formulation.

## THE CLASS-LEVEL LAW IS A THEOREM (proved 2026-08-20, late session)

For every connected G, every S, every projection multiset U:

    N(S,U) = d(S,U) * 2^(|S|-1)

where d(S,U) = #splittings of U into (spanning tree of G) + (S-rooted
forest). Proof: per-edge variables w_e on both copies + rung variables z_u;
prism Laplacian block-diagonalizes as L_w + (L_w + 2Z); tree sum =
(1/2) tau(w) det(L_w + 2Z); all-minors expansion; extract the z_S
coefficient then the w^U coefficient — multilinearity of tree/forest
enumerators makes the RHS coefficient exactly d(S,U). (In the paper as
Theorem "class-level law".) The empirical forms N = d*2^(m-2s) with
s = cycle rank of supp(U) follow from the edge-budget identity
m - 2*rank = |S| - 1 (holds for every class containing a decomposition).
Verified beforehand on all 132 Q3 + 2,459,160 Q4 classes.

Also PROVED: (i) POLARIZATION — every decomposition (T,F) IS a tree of its
class (T on level 0, F on level 1; the component graph is a star), so
d counts the "polarized" trees; (ii) the ACYCLIC-SUPPORT case has a fully
explicit bijection (leaf induction; the |S|-1 free bits are literally the
levels of the singleton edges).

REMAINING OPEN (the only gap to the bijective proof of Stanley's formula):
an explicit straightening map trees-of-class <-> decompositions x
{0,1}^(|S|-1) whose fibers name polarized normal forms. All counting is
now theorem; only the map is missing.

## The support-product THEOREM (upgraded from conjecture 2026-08-19)

PROVEN, for any connected base graph G: #spanning trees of G x K_2 with
vertical support exactly S = (1/2) c(G) 2^|S| F_G(S). Proof: weight
verticals by z_u; the weighted Laplacian block-diagonalizes under the level
swap into L_G and L_G + 2Z; the weighted tree sum is (c(G)/2) det(L_G+2Z);
expand by principal minors + all-minors Matrix-Tree; compare multilinear
coefficients. (Full proof in notes/box-partition-note.tex and
papers/support-product-paper.tex.) The identity is elementary and possibly
folklore (Martin-Reiner / Bernardi Sect. 4 territory); its role is
architectural. Corollaries: (1) iterating over directions telescopes to
Stanley's formula via Pascal; (2) equivalent form: c(G^(S)) =
(1/2) c(G) 2^|S| F_G(S) for the PARTIAL DOUBLE G^(S) (split each vertex
outside S into two copies) — contraction of verticals is the bijection to
partial-double trees; (3) the FULL-SUPPORT slice has an explicit
three-line bijection (contract verticals; free level bit per base tree
edge). Remaining bijective gap, precisely: realize
{trees of G x K2} x {0,1} <-> {trees of G} x {root-2-colored rooted
forests of G} constructively (Problem 1 of the paper).

## The support-product census (discovery data)

Census (`qpark census`): counting (tree, root) pairs of Q_n by the set S of
base vertices carrying a vertical (direction-d) tree edge gives, exactly:

    count(S) = 2^|S| * F_base(S) * 2^(n-1) * c(Q_{n-1})

where F_base(S) = number of S-rooted spanning forests of Q_{n-1} (Laplacian
minor determinant). **Verified exactly, class by class, for n = 2 (3
classes), n = 3 (15 classes), and n = 4 (all 255 classes, from the complete
set of 679,477,248 (tree,root) pairs; 467 s)** — see `q4_census.txt`. Crucially, summing this identity over S via
sum_S z^|S| F_G(S) = det(zI + L_G) at z = 2 **telescopes Stanley's formula
exactly through Pascal's rule** (see notes/box-partition-note.tex,
Prop. "telescope"). So a bijective proof of the census identity at each
level n IS a bijective proof of the hypercube tree formula. This is the
reformulated end goal.

Progress on the mechanism (`scripts/projection_test.py`, Q3 exhaustive):
deleting the vertical edges of a tree leaves horizontal forests in the two
levels; projecting both to the base gives an edge multiset U (multiplicities
1 = "free level choice", 2 = "both levels"). Findings: (i) every occurring U
decomposes as (spanning tree of base) ⊎ (S-rooted forest of base) —
132/132 classes, matching the conjecture's edge budget exactly, and
conversely EVERY decomposable (S, U) occurs; (ii) on Q3 every class obeys
the MASS FORMULA  N(S,U) * d(S,U) = 2^m  exactly (m = #multiplicity-1
"free level" edges, d = #decompositions, N = #trees in the class; the
classes split as d=1 with all 2^m lifts connected, and d=2 with exactly
half). A tree with data (S,U) IS a connected level-assignment of U's free
edges — vertex/edge counts make connected equivalent to tree automatically
— so N counts connected lifts.

Discrimination at n = 4 (`qpark project q4`, all 42.5M trees, 2 459 160
(S,U) classes; `q4_projection.txt`, violators in `q4_violators.txt`): the
mass formula holds in 95.7% of classes — but the exceptional 105 540
classes revealed a UNIVERSAL LAW subsuming everything. In EVERY class of
Q3 and Q4, without exception:

    N(S,U) = d(S,U) * 2^(m - 2s)   for an integer s = s(S,U) >= 0

(verified over all 105 540 exceptional classes, s in {2,3,4,5} there; the
mass formula is the special case d = 2^s; the Q3 dichotomy is s in {0,1}).
Equivalently odd(N) = odd(d) always, and the per-decomposition share of
connected lifts always has EVEN codimension in F_2^m. Geometry probe (40
exceptional classes): in 22, the connected-lift set V is exactly d cosets
of its stabilizer subspace W = {x : V xor x = V} with dim W = m - 2s; in
18, V is fewer larger cosets with decompositions clustering 4 per coset.
Open: prove the universal law, identify s(S,U), and construct the canonical
lift-to-decomposition assignment — that construction IS the per-level
gluing of the bijective program. (The Wilson-sample heuristic "conformer
iff d is a power of two" is nearly but not exactly right: 756 of the
exceptional classes have d a power of two with N·d = 4·2^m.)

## The canonical box partition

**Novelty status (final, after a full citation sweep of Merino 2001 / Chari
1997 / the Gröbner parking-function line / the sandpile literature through
2026):** the general-G box partition appears NEW — no interval partition of
parking functions, no burn-order fibers-as-intervals, no explicit witness
for Merino's Thm 5.8 exists in the literature. Two prior-art anchors to
cite: (1) for COMPLETE graphs the outcome-map fibers of classical parking
functions are known boxes with a product formula (Colmenarejo–Harris et
al., Enumer. Comb. Appl. 1 (2021), Prop. 3.3) — our theorem specializes to
this; (2) Cori–Le Borgne 2003 use the same greedy-burning engine for their
activity-preserving bijection — ours is its fiber-level strengthening. The
Sept 2025 survey-adjacent paper arXiv:2509.11460 (Corry–Dochtermann et al.)
poses even the weaker bijective goal as open (their Question 6.7). Also: no
published algorithm is stated for enumerating maximal parking functions
(poly-delay enumeration of single-source acyclic orientations exists —
Conte–Grossi–Marino–Rizzi 2016/2018 — but the corollary was never drawn),
so qpark's enumerator has standalone algorithmic value.

**Earlier note (2026-08-19, after reading Merino 2001 in full):**
the *existence* of an interval partition of the parking-function multicomplex
with tops the maximal elements is known — C. Merino, "The Chip Firing Game and
Matroid Complexes" (DMTCS Proc. AA, 2001), Theorem 5.8 proves the multicomplex
is M-shellable (settling Chari's conjecture for cographic matroids). Merino's
proof is *inductive* (deletion–contraction over parallel edge classes, gluing
via i-joins) and produces no explicit rule. What is plausibly new here is the
**explicit canonical construction**: the intervals are the fibers of the
lex-minimal-Dhar-burn-order map, with closed-form tops (wcnt − 1) and bottoms
(skip watermarks), indexed by acyclic orientations with unique sink, restricting
to the BCT canonical box on the identity order, and computable at Q4 scale.
Experimentally (Q3, the sample multigraph, 25 random multigraphs): the DFS
emission order (lex order on burn sequences) makes the partition an
**M-shelling**, not just an M-partition — every initial union of boxes is an
order ideal. Conjecturally this holds for all connected multigraphs; a proof
would give an explicit M-shelling where Merino's is recursive.

The enumerator partitions all parking functions into boxes indexed by the
maximal ones. For Q3 (see `q3_box_partition.txt`):

- cell-volume distribution: 1×36, 2×51, 3×12, 4×18, 6×9, 8×3, 12×3, 24×1;
- the unique volume-24 cell is produced by the identity burn order and equals
  the **entire BCT canonical box** dom(f^3) — this holds for every Q_n (the
  identity order never skips, so bottom = 0 and top = wgt − 1), so the
  partition canonically extends BCT Theorem 5.2;
- grouping cells by first deviation from the identity order gives volumes
  24, 160, 110, 24, 48, 10, 8 — *not* multiples of 24, so this particular
  grouping does not explain the 16 × 24 factorization.

## Möbius function of the parking-function poset (Q3)

With a top element adjoined, mu(x, 1) is nonzero for 383 of 384 elements, and
the crosscut identity  −Σ mu(x,1)·vol(dom(x)) = 384  holds, as does BCT
Lemma 4.12 (every PF is the meet of the maximals above it). Values are highly
structured by degree (see `q3_analysis.txt`): constant −1 on the 133 maximals,
{2,3,4} on degree 4, {−6..−3} on degree 3, {2,3,4,6} on degree 2, {0,1,2} on
degree 1, −6 at the bottom. Sign alternation by codegree *fails* at codegrees
4 and 5 — the poset is not as clean as a shellable complex would suggest.

## Orbit census (root-fixing automorphisms = coordinate permutations S_n)

- Q3: 133 maximal PFs fall into 27 orbits (sizes 1×1, 3×8, 6×18); the unique
  fixed point is the canonical PF.
- Q4: 3 040 575 fall into 128 234 orbits (sizes 1×1, 3×2, 4×26, 6×38,
  12×2 981, 24×125 186).

## Negative results (they kill the naive algebraic attack)

Target factorization: #trees(Q_n) = 2^{2^n − n − 1} × vol(canonical box).

1. dom-box volumes over the 133 maximal PFs of Q3 take **only two values**:
   18 (42 of them) and 24 (91 of them).
2. The canonical box maps *injectively* into K(Q3) (24 distinct classes,
   containing 0) but its image is **not a subgroup**.
3. **No order-16 subgroup of K(Q3)** has the canonical box as a transversal
   (exhaustive search over the Sylow-2 of order 128), and the same holds for
   **all 91** volume-24 dom-boxes.
4. K(Q3) has 2-rank 3 < 4 = 2^n − n − 1, so no free (Z/2)^{2^n−n−1} action can
   act by sandpile translations at all.

Conclusion: the 2-power deficit is not visible as a subgroup/coset structure
relative to dom-boxes. The decomposition, if it exists, is combinatorial
rather than translation-algebraic — consistent with Bernardi's proofs working
through spin independence of random forests rather than through the sandpile
group. Next step for Direction 1: translate Bernardi's edge-class spin
statistic across the BCT tree bijection (their Theorem 4.6, which respects the
maximal stratum) and test whether it induces independent Z/2 statistics on
parking functions.

## Files

- `q3_maximal_catalog.txt` — all 133 maximal Q3-parking functions
- `q3_box_partition.txt` — the 133 canonical boxes (order, bottom, top, volume)
- `q3_analysis.txt`, `q4_analysis.txt` — SNF / Möbius / orbits / boxmap output
- `q4_hvector.txt` — T_{Q4}(1,y)
