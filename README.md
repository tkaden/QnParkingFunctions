# QnParkingFunctions

Research project on the maximal G-parking functions of hypercube graphs
Q<sub>n</sub>, aimed at a **bijective proof of Stanley's formula** for the
number of spanning trees of the hypercube:

```
c(Q_n) = 2^(2^n − n − 1) · ∏_{k=1..n} k^C(n,k)
```

Stanley's *Enumerative Combinatorics* vol. 2 (Example 5.6.10) noted that no
direct combinatorial proof of this formula was known. Bernardi (EJC 2012)
gave two combinatorial — but not bijective — proofs; per the second edition
of EC2 (2024, Notes to Ch. 5), a **bijective proof remains open**. That is
this project's target.

## What's here

| Path | Contents |
|------|----------|
| [`qpark/`](qpark/) | Rust tool: enumerates each maximal G-parking function exactly once via canonical (lex-minimal) Dhar burn orders; computes T(1,0), T(1,1), T(1,y), sandpile groups, and runs the research experiments. See [`qpark/README.md`](qpark/README.md). |
| [`qpark/results/`](qpark/results/) | Experimental record ([`RESULTS.md`](qpark/results/RESULTS.md)), the Q₃ catalog and box partition, Q₄ h-vector, K(Q₅), census and projection data, ready-to-submit [OEIS drafts](qpark/results/oeis-drafts.md). |
| [`papers/`](papers/) | Two draft papers (with PDFs): the updated **enumeration paper** (algorithm + fiber theorem + data) and the **support-product paper** (the theorem powering the bijective program). |
| [`notes/`](notes/) | Working note on the canonical box partition / M-shelling. |
| `Draft.pdf` | The original 2016–2019 draft (historical). The original Python and C implementations were removed in 2026; they are available in the git history prior to commit `36335c2`. |

## Main results so far (August 2026)

- **Box partition theorem.** The fibers of the lex-minimal Dhar burn-order
  map are coordinate boxes that *partition* the set of all G-parking
  functions, with box tops exactly the maximal ones. This gives sign-free
  formulas for T(1,1) and T(1,y), a once-per-object enumeration algorithm,
  and an explicit form of Merino's M-shellability theorem for cographic
  matroids. For Q<sub>n</sub>, the identity burn order's cell is exactly the
  Benson–Chakrabarty–Tetali canonical box.
- **Support-product theorem.** For any connected graph G, the spanning trees
  of G×K₂ with vertical support S number ½·c(G)·2^|S|·F_G(S). Iterating it
  over the n directions telescopes to Stanley's formula through Pascal's
  rule. (This yields a new *algebraic* proof of the formula; the *bijective*
  proof — the actual open problem — is NOT yet found. What the theorem does
  is reduce it: one explicit per-level bijection, still to be constructed,
  would finish it by iteration.) The full-support slice of that missing
  bijection is already done explicitly.
- **Class-level law (now a theorem).** Refining by base-projection U: the
  trees in every class (S,U) number exactly d(S,U)·2^(|S|−1), where d counts
  the splittings of U into a base spanning tree plus an S-rooted forest —
  and each splitting appears among the trees as an explicit "polarized"
  representative. All counting in the bijective program is now proven at
  the finest stratum; the single remaining open item is the explicit
  straightening map (Problem 1 of the support-product paper).
- **Data.** Q₄: 3 040 575 maximal parking functions / 42 467 328 spanning
  trees enumerated in ~1.5 s (first direct confirmation of the
  chromatic-polynomial value); T_{Q₄}(1,y) computed (apparently new);
  K(Q₅) = ℤ₂⁵×ℤ₆×ℤ₂₄⁴×ℤ₄₈×ℤ₁₉₂³×ℤ₉₆₀, consistent with all of Bai's
  theorems, including the full (generally open) Sylow-2 part.

## Quick start

```bash
cd qpark
cargo build --release
./target/release/qpark count q4        # maximal PFs + spanning trees, verified
./target/release/qpark hvector q4      # T_{Q4}(1,y)
./target/release/qpark analyze q3      # sandpile SNF, Möbius, orbits
./target/release/qpark validate        # brute-force cross-checks
```

Papers and notes compile with [Tectonic](https://tectonic-typesetting.github.io)
(`tectonic papers/support-product-paper.tex`).

## Open problems

1. Construct the straightening map: an explicit bijection between the trees
   of each class (S,U) and (splittings of U) × {0,1}^(|S|−1), naming each
   tree's polarized normal form (Problem 1 of the support-product paper).
   All counting is proven; only this map is missing, and whoever constructs
   it will have completed the first bijective proof of Stanley's formula.
   (The acyclic-support case is done.)
2. Prove that the box partition, in lex order of burn orders, is an
   M-shelling.

See `papers/support-product-paper.tex` §5 and `qpark/results/RESULTS.md`
for precise statements and the full experimental record.
