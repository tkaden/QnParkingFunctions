//! Brute-force reference implementations used to validate the canonical
//! enumerator on small graphs.

use crate::graph::Graph;

/// Dhar's burning criterion: f (indexed by vertex, root entry ignored) is a
/// parking function iff burning from the root consumes every vertex.
pub fn is_parking(g: &Graph, f: &[u32]) -> bool {
    let n = g.n;
    let mut burnt = 1u64;
    let mut wcnt = vec![0u32; n];
    let mut stack = vec![0usize];
    let mut remaining = n - 1;
    while let Some(v) = stack.pop() {
        let mut m = g.nbr[v] & !burnt;
        while m != 0 {
            let w = m.trailing_zeros() as usize;
            m &= m - 1;
            wcnt[w] += g.adj[v][w];
            if f[w] < wcnt[w] {
                burnt |= 1 << w;
                remaining -= 1;
                stack.push(w);
            }
        }
    }
    remaining == 0
}

/// All parking functions by odometer over the full value box
/// prod_v [0, weighted_degree(v) - 1]. Exponential; small graphs only.
pub fn brute_all(g: &Graph) -> Vec<Vec<u32>> {
    let n = g.n;
    let limits: Vec<u32> = (1..n).map(|v| g.weighted_degree(v)).collect();
    let mut f = vec![0u32; n];
    let mut out = Vec::new();
    loop {
        if is_parking(g, &f) {
            out.push(f[1..].to_vec());
        }
        // odometer increment over positions 1..n
        let mut i = 1;
        loop {
            if i == n {
                return out;
            }
            f[i] += 1;
            if f[i] < limits[i - 1] {
                break;
            }
            f[i] = 0;
            i += 1;
        }
    }
}

/// Domination-maximal elements of a set of vectors.
pub fn maximal_of(set: &[Vec<u32>]) -> Vec<Vec<u32>> {
    set.iter()
        .filter(|a| {
            !set.iter().any(|b| {
                *a != b && a.iter().zip(b.iter()).all(|(x, y)| x <= y)
            })
        })
        .cloned()
        .collect()
}
