//! The Benson–Chakrabarty–Tetali bijection (Discrete Math. 310 (2010), Thm 4.6)
//! between G-parking functions and spanning trees, implemented for SIMPLE
//! graphs, plus Wilson's algorithm for uniform spanning trees and Bernardi
//! spin statistics on hypercubes.
//!
//! Conventions: root q = 0. Trees are parent arrays (parent[0] = -1). A fixed
//! edge order sigma labels edges lexicographically by (min endpoint, max
//! endpoint); "larger" means larger label. The BCT dominance order >_sigma on
//! vertices w.r.t. an arborescence T: i >_sigma j iff the largest edge on the
//! path from i to meet(i,j) exceeds the largest on the path from j to
//! meet(i,j) (empty path = null edge, smaller than everything).

use crate::graph::Graph;

pub fn edge_ids(g: &Graph) -> Vec<Vec<i64>> {
    let n = g.n;
    let mut ids = vec![vec![-1i64; n]; n];
    let mut next = 0i64;
    for u in 0..n {
        for v in u + 1..n {
            if g.adj[u][v] > 0 {
                assert!(g.adj[u][v] == 1, "bij module supports simple graphs only");
                ids[u][v] = next;
                ids[v][u] = next;
                next += 1;
            }
        }
    }
    ids
}

/// Largest edge label on the path from a to meet(a,b), and same from b.
/// Returns (e_ab, e_ba); a path of length zero yields -1 (the null edge).
fn meet_maxima(a: usize, b: usize, parent: &[i32], eids: &[Vec<i64>]) -> (i64, i64) {
    // mark the root path of a
    let mut on_pa = vec![false; parent.len()];
    let mut x = a;
    loop {
        on_pa[x] = true;
        if parent[x] < 0 {
            break;
        }
        x = parent[x] as usize;
    }
    // walk from b to the first marked vertex = meet, tracking max label
    let mut e_ba = -1i64;
    let mut y = b;
    while !on_pa[y] {
        let p = parent[y] as usize;
        e_ba = e_ba.max(eids[y][p]);
        y = p;
    }
    let meet = y;
    // max label from a down to meet
    let mut e_ab = -1i64;
    let mut z = a;
    while z != meet {
        let p = parent[z] as usize;
        e_ab = e_ab.max(eids[z][p]);
        z = p;
    }
    (e_ab, e_ba)
}

/// i >_sigma j in the arborescence given by parent
fn dominates(i: usize, j: usize, parent: &[i32], eids: &[Vec<i64>]) -> bool {
    let (eij, eji) = meet_maxima(i, j, parent, eids);
    eij > eji
}

/// BCT power order of X_u (the neighbors of u inside the labeled set) with
/// respect to the arborescence: returns vertices from most to least powerful.
fn power_order(u: usize, mut xu: Vec<usize>, parent: &[i32], eids: &[Vec<i64>]) -> Vec<usize> {
    let mut out = Vec::with_capacity(xu.len());
    while !xu.is_empty() {
        // x = dominator of xu
        let mut x = xu[0];
        for &v in &xu[1..] {
            if dominates(v, x, parent, eids) {
                x = v;
            }
        }
        // W_u = { v : label(u,v) > largest edge on path x -> meet(x,v) }
        let mut best: Option<usize> = None;
        for &v in &xu {
            let (exv, _) = meet_maxima(x, v, parent, eids);
            if eids[u][v] > exv {
                if best.map_or(true, |b| eids[u][v] > eids[u][b]) {
                    best = Some(v);
                }
            }
        }
        let vstar = best.expect("W_u is nonempty (contains the dominator)");
        out.push(vstar);
        xu.retain(|&v| v != vstar);
    }
    out
}

/// BCT pi: parking function -> spanning tree (parent array).
pub fn pi(g: &Graph, eids: &[Vec<i64>], f: &[u32]) -> Vec<i32> {
    let n = g.n;
    let mut parent = vec![-1i32; n];
    let mut in_x = vec![false; n];
    in_x[0] = true;
    for _ in 1..n {
        // S = unlabeled vertices that are over-threshold
        let s: Vec<usize> = (1..n)
            .filter(|&v| !in_x[v])
            .filter(|&v| {
                let cnt = (0..n).filter(|&w| in_x[w] && g.adj[v][w] > 0).count() as u32;
                cnt > f[v]
            })
            .collect();
        assert!(!s.is_empty(), "pi: input is not a parking function");
        // tentative connection M(u) for each u in S
        let mut m = vec![0usize; n];
        for &u in &s {
            let xu: Vec<usize> = (0..n).filter(|&w| in_x[w] && g.adj[u][w] > 0).collect();
            let order = power_order(u, xu.clone(), &parent, eids);
            m[u] = order[xu.len() - f[u] as usize - 1]; // (|X_u| - f(u))-th, 1-indexed
        }
        // T' = current arborescence plus all tentative edges
        let mut tprime = parent.clone();
        for &u in &s {
            tprime[u] = m[u] as i32;
        }
        // u* = element of S dominated by all others in S (w.r.t. T')
        let mut ustar = s[0];
        for &u in &s[1..] {
            if dominates(ustar, u, &tprime, eids) {
                ustar = u;
            }
        }
        parent[ustar] = m[ustar] as i32;
        in_x[ustar] = true;
    }
    parent
}

/// BCT mu: spanning tree -> parking function. Deletes the externally active
/// non-tree edges (largest in their fundamental cycle), orients every
/// remaining edge from the dominant endpoint to the dominated one (tree edges
/// then point toward the root), and sets f(v) = outdegree(v) - 1.
pub fn mu(g: &Graph, eids: &[Vec<i64>], parent: &[i32]) -> Vec<u32> {
    let n = g.n;
    let mut outdeg = vec![0u32; n];
    for v in 1..n {
        outdeg[v] += 1; // tree edge toward the root
    }
    for u in 0..n {
        for v in u + 1..n {
            if g.adj[u][v] > 0 && parent[u] != v as i32 && parent[v] != u as i32 {
                let (euv, evu) = meet_maxima(u, v, parent, eids);
                let path_max = euv.max(evu);
                if eids[u][v] > path_max {
                    continue; // externally active: deleted
                }
                if euv > evu {
                    outdeg[u] += 1; // u dominates v
                } else {
                    outdeg[v] += 1;
                }
            }
        }
    }
    (0..n).map(|v| outdeg[v] - u32::from(v != 0)).collect()
}

/// Wilson's algorithm: uniform spanning tree rooted at 0 (parent array).
pub fn wilson(g: &Graph, rng: &mut u64) -> Vec<i32> {
    fn next_rand(x: &mut u64) -> u64 {
        *x = x.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = *x;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    let n = g.n;
    let mut in_tree = vec![false; n];
    let mut nxt = vec![-1i32; n];
    in_tree[0] = true;
    let nbrs: Vec<Vec<usize>> = (0..n)
        .map(|v| (0..n).filter(|&w| g.adj[v][w] > 0).collect())
        .collect();
    for start in 1..n {
        if in_tree[start] {
            continue;
        }
        // loop-erased random walk to the tree
        let mut v = start;
        while !in_tree[v] {
            let ns = &nbrs[v];
            nxt[v] = ns[(next_rand(rng) % ns.len() as u64) as usize] as i32;
            v = nxt[v] as usize;
        }
        let mut v = start;
        while !in_tree[v] {
            in_tree[v] = true;
            v = nxt[v] as usize;
        }
    }
    nxt
}

/// For a hypercube tree rooted at 0: for each non-root vertex v, its out-edge
/// (toward the root) has direction i = the differing coordinate and spin =
/// the i-th coordinate of the PARENT (Bernardi's convention: the value the
/// edge points to).
pub fn out_edge_spins(dim: u32, parent: &[i32]) -> Vec<(u32, u8)> {
    let n = 1usize << dim;
    (1..n)
        .map(|v| {
            let p = parent[v] as usize;
            let i = (v ^ p).trailing_zeros();
            (i, ((p >> i) & 1) as u8)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dhar, exact};

    fn roundtrip_all(g: &Graph, label: &str) {
        let eids = edge_ids(g);
        let all = dhar::brute_all(g);
        let mut trees: Vec<Vec<i32>> = Vec::new();
        for fv in &all {
            let mut f = vec![0u32; g.n];
            f[1..].copy_from_slice(fv);
            let t = pi(g, &eids, &f);
            // valid arborescence rooted at 0
            for v in 1..g.n {
                assert!(t[v] >= 0 && g.adj[v][t[v] as usize] > 0, "{label}: bad parent");
            }
            let back = mu(g, &eids, &t);
            assert_eq!(back[1..].to_vec(), *fv, "{label}: mu(pi(f)) != f");
            trees.push(t);
        }
        trees.sort();
        trees.dedup();
        assert_eq!(trees.len() as u128, exact::kirchhoff(g), "{label}: pi not onto trees");
    }

    #[test]
    fn bct_bijection_small() {
        roundtrip_all(&Graph::hypercube(2), "q2");
        roundtrip_all(&Graph::hypercube(3), "q3");
    }

    #[test]
    fn bct_bijection_random_simple() {
        // seeded random simple connected graphs
        let mut seed = 424242u64;
        let mut next = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (seed >> 33) as u32
        };
        let mut done = 0;
        while done < 12 {
            let n = 4 + (next() % 3) as usize;
            let mut adj = vec![vec![0u32; n]; n];
            for i in 0..n {
                for j in i + 1..n {
                    if next() % 3 > 0 {
                        adj[i][j] = 1;
                        adj[j][i] = 1;
                    }
                }
            }
            if let Ok(g) = Graph::from_matrix(adj) {
                roundtrip_all(&g, &format!("rand{done}"));
                done += 1;
            }
        }
    }

    #[test]
    fn wilson_produces_trees() {
        let g = Graph::hypercube(3);
        let eids = edge_ids(&g);
        let mut rng = 7u64;
        for _ in 0..200 {
            let t = wilson(&g, &mut rng);
            for v in 1..g.n {
                assert!(t[v] >= 0 && g.adj[v][t[v] as usize] > 0);
            }
            // tree -> PF -> tree roundtrip
            let f = mu(&g, &eids, &t);
            let t2 = pi(&g, &eids, &f);
            let f2 = mu(&g, &eids, &t2);
            assert_eq!(f, f2);
        }
    }
}
