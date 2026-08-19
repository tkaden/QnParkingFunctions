# OEIS submission drafts (prepared 2026-08-19)

Ready to paste into https://oeis.org/Submit — needs a (free) OEIS account.
Submit under your own name; adjust wording freely.

## Draft 1: number of maximal parking functions of the hypercube

**Data:** 1, 3, 133, 3040575, 81768640551939777

**Offset:** 1

**Name:** Number of maximal G-parking functions of the n-dimensional
hypercube graph Q_n (equivalently, acyclic orientations of Q_n with a unique
fixed sink; equivalently, the Tutte evaluation T_{Q_n}(1,0)).

**Comments:**
- a(n) = |[x^1] P(Q_n, x)|, the absolute value of the linear coefficient of
  the chromatic polynomial of Q_n (Greene-Zaslavsky), independent of the
  choice of sink.
- Also the number of maximal superstable configurations of the abelian
  sandpile model on Q_n, and the number of spanning trees of Q_n with
  external activity 0 ("safe trees") for any fixed edge order.
- a(2..4) computed by Benson, Chakrabarty and Tetali (2010); a(5) follows
  from the chromatic polynomial of Q_5 computed by Andrew Howroyd in
  A334278 (2020). Values a(2..4) reproduced by direct enumeration (see Links).
- The number of ALL G-parking functions of Q_n is the number of spanning
  trees, A006237.

**Formula:** a(n) = |A334278(n, 1)| (linear coefficient of the chromatic
polynomial of Q_n).

**Cross-references:** A006237 (spanning trees of Q_n), A334278 (chromatic
polynomial coefficients of Q_n), A334247 (acyclic orientations of Q_n).

**References:**
- B. Benson, D. Chakrabarty, P. Tetali, G-parking functions, acyclic
  orientations and spanning trees, Discrete Math. 310 (2010), 1340-1353.
- C. Greene, T. Zaslavsky, On the interpretation of Whitney numbers...,
  Trans. Amer. Math. Soc. 280 (1983), 97-126.

## Draft 2: T_{Q_4}(1,y) / parking functions of Q_4 by degree

**Data (row):** 1, 15, 120, 680, 3044, 11388, 36808, 104984, 267894, 616906,
1287688, 2436504, 4156516, 6306364, 8278008, 8903016, 7016817, 3040575

**Name:** Number of G-parking functions (superstable configurations) of the
4-dimensional hypercube Q_4 by total degree k, 0 <= k <= 17; equivalently
the coefficients of y^(17-k) in T_{Q_4}(1,y); equivalently the h-vector of
the cographic matroid complex of Q_4.

**Comments:**
- Row sum is 42467328 = A006237(4), the number of spanning trees of Q_4;
  the top entry 3040575 is T_{Q_4}(1,0).
- By Merino's theorem this is a pure O-sequence (Stanley's conjecture holds
  for cographic matroids).
- Could be submitted as a finite sequence, or as motivation to extend to a
  triangle "T(n,k) = number of parking functions of Q_n of degree k" with
  rows n = 1..4 (row n has length g+1 = n*2^(n-1) - 2^n + 2):
  n=1: 1; n=2: 1, 3; n=3: 1, 7, 28, 76, 139, 133; n=4: the row above.

**References:**
- C. Merino Lopez, Chip firing and the Tutte polynomial, Ann. Comb. 1
  (1997), 253-259.

## Draft 3: comment to add to A006237 (spanning trees of n-cube)

Proposed comment:
"A combinatorial (though not bijective) proof of the product formula was
given by O. Bernardi, On the spanning trees of the hypercube and other
products of graphs, Electron. J. Combin. 19(4) (2012), #P51, answering a
question of Stanley (Enumerative Combinatorics 2, Example 5.6.10). A fully
bijective proof remains open (Stanley, EC2 2nd ed., 2024, Notes to Ch. 5)."

Proposed link:
"O. Bernardi, <a href=\"https://doi.org/10.37236/2510\">On the spanning
trees of the hypercube and other products of graphs</a>, Electron. J.
Combin. 19(4) (2012), #P51."
