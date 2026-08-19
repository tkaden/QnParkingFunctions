//! Closed formulas and determinant cross-checks.

use crate::graph::Graph;

fn binom(n: u32, k: u32) -> u32 {
    let mut r: u64 = 1;
    for i in 0..k {
        r = r * (n - i) as u64 / (i + 1) as u64;
    }
    r as u32
}

/// Stanley's formula (EC2, Example 5.6.10): spanning trees of Q_n.
pub fn stanley_qn_trees(n: u32) -> u128 {
    assert!((1..=6).contains(&n));
    let mut t: u128 = 1u128 << ((1u32 << n) - n - 1);
    for k in 2..=n {
        t *= (k as u128).pow(binom(n, k));
    }
    t
}

/// Number of spanning trees via the Matrix-Tree theorem: determinant of the
/// reduced Laplacian (root row/column removed), fraction-free Bareiss in i128.
pub fn kirchhoff(g: &Graph) -> u128 {
    let m = g.n - 1;
    let mut a = vec![vec![0i128; m]; m];
    for i in 0..m {
        for j in 0..m {
            a[i][j] = if i == j {
                g.weighted_degree(i + 1) as i128
            } else {
                -(g.adj[i + 1][j + 1] as i128)
            };
        }
    }
    let mut sign = 1i128;
    let mut prev = 1i128;
    for k in 0..m {
        if a[k][k] == 0 {
            let Some(p) = (k + 1..m).find(|&i| a[i][k] != 0) else {
                return 0;
            };
            a.swap(k, p);
            sign = -sign;
        }
        for i in k + 1..m {
            for j in k + 1..m {
                a[i][j] = (a[i][j] * a[k][k] - a[i][k] * a[k][j]) / prev;
            }
            a[i][k] = 0;
        }
        prev = a[k][k];
    }
    (sign * a[m - 1][m - 1]) as u128
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn stanley_values() {
        assert_eq!(stanley_qn_trees(1), 1);
        assert_eq!(stanley_qn_trees(2), 4);
        assert_eq!(stanley_qn_trees(3), 384);
        assert_eq!(stanley_qn_trees(4), 42_467_328);
        assert_eq!(stanley_qn_trees(5), 20_776_019_874_734_407_680);
    }

    #[test]
    fn kirchhoff_matches_stanley() {
        for n in 2..=4 {
            assert_eq!(kirchhoff(&Graph::hypercube(n)), stanley_qn_trees(n));
        }
    }
}
