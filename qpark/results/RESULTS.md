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

## The canonical box partition (new object)

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
