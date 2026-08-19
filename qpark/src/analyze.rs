//! Phase-2 experiments: sandpile group structure (Smith normal form), the
//! Möbius function of the parking-function poset, automorphism orbit census,
//! and the image of the BCT canonical box inside the sandpile group.

use crate::dhar;
use crate::graph::Graph;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------------------
// Smith normal form of the reduced Laplacian, tracking the left transform U,
// so that configurations map to sandpile-group elements via x -> (U x) mod d.
// ---------------------------------------------------------------------------

pub struct SandpileGroup {
    /// all invariant factors (including trivial 1s), diagonal of the SNF
    pub factors: Vec<i128>,
    /// left unimodular transform: U * L' * V = diag(factors)
    pub u: Vec<Vec<i128>>,
}

pub fn reduced_laplacian(g: &Graph) -> Vec<Vec<i128>> {
    let m = g.n - 1;
    (0..m)
        .map(|i| {
            (0..m)
                .map(|j| {
                    if i == j {
                        g.weighted_degree(i + 1) as i128
                    } else {
                        -(g.adj[i + 1][j + 1] as i128)
                    }
                })
                .collect()
        })
        .collect()
}

pub fn sandpile_group(g: &Graph) -> SandpileGroup {
    let mut a = reduced_laplacian(g);
    let m = a.len();
    let mut u: Vec<Vec<i128>> = (0..m)
        .map(|i| (0..m).map(|j| i128::from(i == j)).collect())
        .collect();

    for t in 0..m {
        loop {
            // locate pivot: nonzero entry of minimal absolute value in the submatrix
            let mut best: Option<(usize, usize)> = None;
            for i in t..m {
                for j in t..m {
                    if a[i][j] != 0
                        && best.map_or(true, |(bi, bj)| a[i][j].abs() < a[bi][bj].abs())
                    {
                        best = Some((i, j));
                    }
                }
            }
            let Some((pi, pj)) = best else {
                return finish(a, u);
            };
            a.swap(t, pi);
            u.swap(t, pi);
            for row in a.iter_mut() {
                row.swap(t, pj);
            }

            let mut clean = true;
            for i in t + 1..m {
                if a[i][t] != 0 {
                    let q = a[i][t] / a[t][t];
                    for j in 0..m {
                        a[i][j] -= q * a[t][j];
                        u[i][j] -= q * u[t][j];
                    }
                    if a[i][t] != 0 {
                        clean = false;
                    }
                }
            }
            if !clean {
                continue;
            }
            for j in t + 1..m {
                if a[t][j] != 0 {
                    let q = a[t][j] / a[t][t];
                    for row in a.iter_mut() {
                        row[j] -= q * row[t];
                    }
                    if a[t][j] != 0 {
                        clean = false;
                    }
                }
            }
            if !clean {
                continue;
            }
            // divisibility: d_t must divide every remaining entry
            let mut fixed = true;
            'div: for i in t + 1..m {
                for j in t + 1..m {
                    if a[i][j] % a[t][t] != 0 {
                        for k in 0..m {
                            a[t][k] += a[i][k];
                            u[t][k] += u[i][k];
                        }
                        fixed = false;
                        break 'div;
                    }
                }
            }
            if fixed {
                break;
            }
        }
    }
    finish(a, u)
}

fn finish(a: Vec<Vec<i128>>, mut u: Vec<Vec<i128>>) -> SandpileGroup {
    let m = a.len();
    let mut factors: Vec<i128> = (0..m).map(|i| a[i][i]).collect();
    for i in 0..m {
        if factors[i] < 0 {
            factors[i] = -factors[i];
            for x in u[i].iter_mut() {
                *x = -*x;
            }
        }
    }
    SandpileGroup { factors, u }
}

impl SandpileGroup {
    pub fn order(&self) -> u128 {
        self.factors.iter().map(|&d| d as u128).product()
    }

    /// nontrivial invariant factors (> 1), smallest first
    pub fn invariants(&self) -> Vec<i128> {
        let mut v: Vec<i128> = self.factors.iter().copied().filter(|&d| d > 1).collect();
        v.sort();
        v
    }

    /// class of a configuration on the non-root vertices, as a mixed-radix key
    pub fn class_key(&self, x: &[u32]) -> u128 {
        let m = self.factors.len();
        let mut key: u128 = 0;
        let mut stride: u128 = 1;
        for i in 0..m {
            let d = self.factors[i];
            if d <= 1 {
                continue;
            }
            let mut s: i128 = 0;
            for j in 0..m {
                s += self.u[i][j] * x[j] as i128;
            }
            let c = s.rem_euclid(d);
            key += c as u128 * stride;
            stride *= d as u128;
        }
        key
    }

    /// componentwise sum of two class keys (group addition)
    pub fn key_add(&self, a: u128, b: u128) -> u128 {
        let mut out: u128 = 0;
        let mut stride: u128 = 1;
        let (mut ra, mut rb) = (a, b);
        for &d in self.factors.iter().filter(|&&d| d > 1) {
            let d = d as u128;
            let ca = ra % d;
            let cb = rb % d;
            out += ((ca + cb) % d) * stride;
            stride *= d;
            ra /= d;
            rb /= d;
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Möbius function of the parking-function poset (domination order) with a top
// element adjoined; crosscut identity  |P| = -sum_x mu(x, 1) * vol(dom(x)).
// ---------------------------------------------------------------------------

pub struct MobiusReport {
    pub total: usize,
    pub maximal: usize,
    pub nonzero_mu: usize,
    pub identity_lhs: i128,
    /// (degree, mu value) -> count
    pub by_degree: HashMap<(u32, i128), usize>,
    pub lemma_412_holds: bool,
}

pub fn mobius_experiment(g: &Graph) -> MobiusReport {
    let all = dhar::brute_all(g);
    let n1 = g.n - 1;
    let leq = |a: &[u32], b: &[u32]| a.iter().zip(b.iter()).all(|(x, y)| x <= y);

    let deg: Vec<u32> = all.iter().map(|f| f.iter().sum()).collect();
    let mut idx: Vec<usize> = (0..all.len()).collect();
    idx.sort_by(|&i, &j| deg[j].cmp(&deg[i])); // degree descending

    let mut mu = vec![0i128; all.len()];
    for (pos, &i) in idx.iter().enumerate() {
        let mut s: i128 = 1; // mu(1hat, 1hat)
        for &j in &idx[..pos] {
            if deg[j] > deg[i] || (deg[j] == deg[i] && all[j] != all[i]) {
                if all[i] != all[j] && leq(&all[i], &all[j]) {
                    s += mu[j];
                }
            }
        }
        mu[i] = -s;
    }

    let vol = |f: &[u32]| -> i128 { f.iter().map(|&x| (x + 1) as i128).product() };
    let identity_lhs: i128 = -(0..all.len()).map(|i| mu[i] * vol(&all[i])).sum::<i128>();

    let genus = g.genus() as u32;
    let maximal = deg.iter().filter(|&&d| d == genus).count();

    // Lemma 4.12 (BCT): every parking function is the meet of the maximal ones above it
    let maxima: Vec<&Vec<u32>> = all.iter().filter(|f| f.iter().sum::<u32>() == genus).collect();
    let lemma_412_holds = all.iter().all(|f| {
        let mut meet = vec![u32::MAX; n1];
        for m in maxima.iter().filter(|m| leq(f, m)) {
            for (a, b) in meet.iter_mut().zip(m.iter()) {
                *a = (*a).min(*b);
            }
        }
        meet == **f
    });

    let mut by_degree: HashMap<(u32, i128), usize> = HashMap::new();
    for i in 0..all.len() {
        *by_degree.entry((deg[i], mu[i])).or_insert(0) += 1;
    }

    MobiusReport {
        total: all.len(),
        maximal,
        nonzero_mu: mu.iter().filter(|&&m| m != 0).count(),
        identity_lhs,
        by_degree,
        lemma_412_holds,
    }
}

// ---------------------------------------------------------------------------
// Orbit census of the maximal parking functions under the root-fixing
// automorphisms of Q_dim (coordinate permutations, order dim!).
// ---------------------------------------------------------------------------

fn permutations(k: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut p: Vec<usize> = (0..k).collect();
    fn heap(k: usize, p: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if k == 1 {
            out.push(p.clone());
            return;
        }
        for i in 0..k {
            heap(k - 1, p, out);
            if k % 2 == 0 {
                p.swap(i, k - 1);
            } else {
                p.swap(0, k - 1);
            }
        }
    }
    heap(k, &mut p, &mut out);
    out
}

/// Pack a parking function on 2^dim - 1 non-root vertices, values < 4, into u64.
pub fn pack(f: &[u32]) -> u64 {
    f.iter().enumerate().fold(0u64, |acc, (i, &x)| {
        debug_assert!(x < 4);
        acc | (x as u64) << (2 * i)
    })
}

fn unpack(key: u64, len: usize, out: &mut [u32]) {
    for (i, o) in out.iter_mut().enumerate().take(len) {
        *o = (key >> (2 * i) & 3) as u32;
    }
}

pub struct OrbitReport {
    pub items: usize,
    pub orbits: usize,
    /// orbit size -> number of orbits of that size
    pub size_histogram: Vec<(usize, usize)>,
}

pub fn orbit_census(dim: u32, packed_maximals: &[u64]) -> OrbitReport {
    let n1 = (1usize << dim) - 1;
    let perms = permutations(dim as usize);
    // vertex maps: image of vertex v under each coordinate permutation
    let vmaps: Vec<Vec<usize>> = perms
        .iter()
        .map(|p| {
            (0..=n1)
                .map(|v| {
                    let mut w = 0usize;
                    for (b, &pb) in p.iter().enumerate() {
                        if v >> b & 1 == 1 {
                            w |= 1 << pb;
                        }
                    }
                    w
                })
                .collect()
        })
        .collect();

    let canon: Vec<u64> = packed_maximals
        .par_iter()
        .map(|&key| {
            let mut f = [0u32; 63];
            unpack(key, n1, &mut f);
            let mut best = u64::MAX;
            for vm in &vmaps {
                let mut img = 0u64;
                for v in 1..=n1 {
                    img |= (f[v - 1] as u64) << (2 * (vm[v] - 1));
                }
                best = best.min(img);
            }
            best
        })
        .collect();

    let mut sorted = canon;
    sorted.par_sort_unstable();
    let mut orbits = 0usize;
    let mut sizes: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;
    while i < sorted.len() {
        let mut j = i;
        while j < sorted.len() && sorted[j] == sorted[i] {
            j += 1;
        }
        orbits += 1;
        *sizes.entry(j - i).or_insert(0) += 1;
        i = j;
    }
    let mut size_histogram: Vec<(usize, usize)> = sizes.into_iter().collect();
    size_histogram.sort();
    OrbitReport { items: sorted.len(), orbits, size_histogram }
}

// ---------------------------------------------------------------------------
// The BCT canonical box dom(f^n), f^n(v) = wgt(v) - 1, mapped into the
// sandpile group: are its classes distinct, and do they form a subgroup?
// ---------------------------------------------------------------------------

pub struct BoxmapReport {
    pub box_volume: u128,
    pub distinct_classes: usize,
    pub contains_zero: bool,
    pub is_subgroup: Option<bool>,
    pub group_order: u128,
    pub two_rank: usize,
}

pub fn boxmap_experiment(g: &Graph, dim: u32, sp: &SandpileGroup) -> BoxmapReport {
    let n = g.n;
    let ranges: Vec<u32> = (1..n).map(|v| (v as u32).count_ones()).collect();
    let box_volume: u128 = ranges.iter().map(|&r| r as u128).product();
    assert_eq!(n, 1usize << dim);

    // odometer over the box, collecting class keys
    let mut classes: HashSet<u128> = HashSet::with_capacity(box_volume as usize);
    let mut f = vec![0u32; n - 1];
    loop {
        classes.insert(sp.class_key(&f));
        let mut i = 0;
        loop {
            if i == n - 1 {
                let distinct = classes.len();
                let contains_zero = classes.contains(&sp.class_key(&vec![0u32; n - 1]));
                let is_subgroup = subgroup_check(sp, &classes);
                let two_rank = sp.invariants().iter().filter(|&&d| d % 2 == 0).count();
                return BoxmapReport {
                    box_volume,
                    distinct_classes: distinct,
                    contains_zero,
                    is_subgroup,
                    group_order: sp.order(),
                    two_rank,
                };
            }
            f[i] += 1;
            if f[i] < ranges[i] {
                break;
            }
            f[i] = 0;
            i += 1;
        }
    }
}

/// Closure under addition; None if the check was skipped for size.
fn subgroup_check(sp: &SandpileGroup, classes: &HashSet<u128>) -> Option<bool> {
    let items: Vec<u128> = classes.iter().copied().collect();
    let m = items.len();
    if m as u128 * m as u128 > 2_000_000_000 {
        return None;
    }
    for (i, &a) in items.iter().enumerate() {
        for &b in &items[i..] {
            if !classes.contains(&sp.key_add(a, b)) {
                return Some(false);
            }
        }
    }
    Some(true)
}
