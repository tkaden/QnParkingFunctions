## What it computes

For a connected multigraph G (root q = vertex 0), one search pass produces:

- every **maximal G-parking function**, each exactly once (count = T_G(1,0));
- the number of **spanning trees** (= number of all G-parking functions, T_G(1,1));
- the full **degree enumerator** of parking functions, i.e. the Tutte
  coefficients T_G(1,y) (Merino's theorem) — the h-vector of the cographic matroid;
- a **canonical box partition** of the set of all parking functions, one box per
  maximal parking function (see below).

Q4 (3 040 575 maximal parking functions, 42 467 328 spanning trees) takes about
1.5 s. Q5 has ~8.2 × 10^16 maximal parking functions and cannot be listed by
anything; use the chromatic polynomial of Q5 (OEIS A334278) for its count.

## Usage

```
cargo build --release
./target/release/qpark count   q4          # counts + Matrix-Tree + Stanley checks
./target/release/qpark hvector q4          # parking functions by degree, T(1,y)
./target/release/qpark list-max q3 out.txt # catalog of maximal parking functions
./target/release/qpark boxes   q3 out.txt  # the canonical box partition
./target/release/qpark analyze q3          # sandpile SNF, Mobius, orbits, box map
./target/release/qpark estimate q5 5000000 # Knuth random-descent count estimates
./target/release/qpark spins q3            # Bernardi spin experiment (exhaustive)
./target/release/qpark spins q5 1000000    # same, Wilson-sampled uniform trees
./target/release/qpark validate            # brute-force cross-checks
```

Graphs: `q1`..`q6` (hypercubes), or a file containing `n` followed by an
`n x n` symmetric multiplicity matrix (whitespace-separated).

## The algorithm

Every parking function f has a unique **lex-minimal Dhar burn order**: starting
from the root, repeatedly burn the smallest vertex v with
f(v) < (weight of burnt neighbors). The search enumerates exactly the orders
arising this way, once each, without fixing f in advance: burning v at a step
where its burnt-neighbor weight is `wcnt` and its watermark (burnt-neighbor
weight at the last moment a larger vertex was preferred over it) is `w2`
commits f(v) to the interval `[w2, wcnt - 1]`. Hence each leaf of the search is
a **box** of parking functions, the boxes **partition** the set of all parking
functions, and each box top is a distinct maximal parking function (every
maximal one occurring). No deduplication structure is needed — the trie of
`FinderParallel.c` and the terminal `set()` of the Python version were
compensating for enumerating all burn orders instead of one canonical order
per object.

The identity burn order is always canonical and its box is exactly
`dom(f^n)` for the BCT canonical parking function f^n(v) = wgt(v) − 1
(Benson–Chakrabarty–Tetali, Discrete Math. 310 (2010), Theorem 5.2), so the
partition canonically extends BCT's single-box result to all of P(G, q).

## Validation

`cargo test` / `qpark validate` check, on Q1–Q3, the multigraph example from
the 2016 Python code, and dozens of seeded random connected multigraphs:

- the box union equals the brute-force (odometer + Dhar) parking-function set,
  with no overlaps between boxes;
- box tops equal the brute-force domination-maximal set, all of degree
  g = |E| − |V| + 1;
- spanning-tree counts match Matrix-Tree determinants (and Stanley's formula
  on hypercubes);
- Smith normal form of the reduced Laplacian: group order = tree count,
  parking functions land in pairwise distinct sandpile-group classes,
  invariant-factor count on Q3 matches Bai's theorem;
- the Möbius crosscut identity and BCT Lemma 4.12 on Q2/Q3.

Experimental results on Q3/Q4 live in `results/` — see `results/RESULTS.md`.
