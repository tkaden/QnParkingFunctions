mod analyze;
mod bij;
mod burn;
mod dhar;
mod exact;
mod graph;

use graph::Graph;
use std::io::Write as _;
use std::time::Instant;

const USAGE: &str = "\
qpark — canonical enumeration of maximal G-parking functions

USAGE:
  qpark count   <graph>          leaves (=maximal PFs, T(1,0)) and spanning trees
  qpark hvector <graph>          parking functions by degree; T_G(1,y) coefficients
  qpark list-max <graph> [FILE]  every maximal parking function, one per line
  qpark boxes   <graph> [FILE]   the canonical box partition: order | bottom | top
  qpark analyze <graph> [EXPT]   snf | mobius | orbits | boxmap | all (default all)
  qpark validate                 brute-force cross-checks on small graphs

GRAPH: q1..q6 (hypercube) or a file: first token n, then n*n multiplicities.";

fn parse_graph(spec: &str) -> Result<(Graph, Option<u32>), String> {
    if spec.len() <= 2 {
        if let Some(d) = spec.strip_prefix(['q', 'Q']) {
            if let Ok(dim) = d.parse::<u32>() {
                if !(1..=6).contains(&dim) {
                    return Err("hypercube dimension must be 1..=6".into());
                }
                return Ok((Graph::hypercube(dim), Some(dim)));
            }
        }
    }
    let text = std::fs::read_to_string(spec).map_err(|e| format!("cannot read {spec}: {e}"))?;
    let nums: Vec<u32> = text
        .split_whitespace()
        .map(|t| t.parse::<u32>().map_err(|_| format!("bad number: {t}")))
        .collect::<Result<_, _>>()?;
    let n = *nums.first().ok_or("empty graph file")? as usize;
    if nums.len() != 1 + n * n {
        return Err(format!("expected {} numbers after n={n}, got {}", n * n, nums.len() - 1));
    }
    let adj: Vec<Vec<u32>> = (0..n).map(|i| nums[1 + i * n..1 + (i + 1) * n].to_vec()).collect();
    Ok((Graph::from_matrix(adj)?, None))
}

fn fmt_pf(top: &[u32]) -> String {
    let mut s = String::from("[-1");
    for v in 1..top.len() {
        s.push_str(&format!(", {}", top[v]));
    }
    s.push(']');
    s
}

fn cmd_count(g: &Graph, dim: Option<u32>) {
    if g.n > 20 {
        eprintln!(
            "note: leaf count equals T(1,0); for large graphs this can be astronomically many \
             (Q5 has ~8.2e16 maximal parking functions — do not wait for that)."
        );
    }
    let t0 = Instant::now();
    let agg = burn::aggregate(g);
    let dt = t0.elapsed();
    println!("vertices              : {}", g.n);
    println!("weighted edges        : {}", g.weighted_edges());
    println!("genus g = |E|-|V|+1   : {}", g.genus());
    println!("maximal PFs = T(1,0)  : {}", agg.leaves);
    println!("spanning trees T(1,1) : {}", agg.trees);
    let kirchhoff = exact::kirchhoff(g);
    println!("Matrix-Tree check     : {} {}", kirchhoff, ok(agg.trees == kirchhoff));
    if let Some(d) = dim {
        let stanley = exact::stanley_qn_trees(d);
        println!("Stanley formula check : {} {}", stanley, ok(agg.trees == stanley));
    }
    println!("elapsed               : {dt:.2?}");
}

fn cmd_estimate(g: &Graph, dim: Option<u32>, samples: u64) {
    // reference maximal-PF counts from published chromatic polynomials (OEIS A334278)
    const CHROMATIC_T10: [u128; 6] = [0, 1, 3, 133, 3_040_575, 81_768_640_551_939_777];
    let t0 = Instant::now();
    let e = burn::estimate(g, samples, 0x5eed_2026_0819);
    println!("samples               : {}", e.samples);
    println!(
        "maximal PFs (est)     : {:.4e}  +- {:.1e}  (rel err {:.2}%)",
        e.leaves_mean,
        e.leaves_stderr,
        100.0 * e.leaves_stderr / e.leaves_mean
    );
    if let Some(d) = dim {
        let r = CHROMATIC_T10[d as usize] as f64;
        println!(
            "  reference (chromatic): {:.4e}  -> estimate off by {:+.2} standard errors",
            r,
            (e.leaves_mean - r) / e.leaves_stderr
        );
    }
    println!(
        "spanning trees (est)  : {:.4e}  +- {:.1e}  (rel err {:.2}%)",
        e.trees_mean,
        e.trees_stderr,
        100.0 * e.trees_stderr / e.trees_mean
    );
    if let Some(d) = dim {
        let r = exact::stanley_qn_trees(d) as f64;
        println!(
            "  reference (Stanley)  : {:.4e}  -> estimate off by {:+.2} standard errors",
            r,
            (e.trees_mean - r) / e.trees_stderr
        );
    }
    println!("elapsed               : {:.2?}", t0.elapsed());
}

/// Mass-formula experiment: classify spanning trees of Q_dim by
/// (S, U) where S = direction-0 vertical support and U = multiset projection
/// of the horizontal edges onto the base Q_{dim-1} (multiplicity 1 = free
/// level choice, 2 = both levels). Then test, for every class, the competing
/// laws  N * d = 2^m  vs  N = d * 2^(m-2)  where N = #trees in the class,
/// d = #decompositions of U into (base spanning tree) + (S-rooted forest),
/// and m = #multiplicity-1 edges. Exact for dim <= 4.
fn cmd_project(g: &Graph, dim: u32) {
    use rayon::prelude::*;
    use std::collections::HashMap;
    let t0 = Instant::now();
    let n = g.n;
    let bn = n / 2;
    let eids = bij::edge_ids(g);

    // base edge indexing: Q_{dim-1} on compressed labels u = v >> 1
    let base = Graph::hypercube(dim - 1);
    let mut be_id = vec![vec![usize::MAX; bn]; bn];
    let mut base_edges = Vec::new();
    for u in 0..bn {
        for v in u + 1..bn {
            if base.adj[u][v] > 0 {
                be_id[u][v] = base_edges.len();
                be_id[v][u] = base_edges.len();
                base_edges.push((u, v));
            }
        }
    }
    let ne = base_edges.len();
    assert!(2 * ne <= 64, "U encoding needs 2 bits per base edge");

    // classify all trees by (S, U)
    let mut boxes: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
    burn::enumerate(g, &mut |b| boxes.push((b.bottom.to_vec(), b.top.to_vec())));
    let maps: Vec<HashMap<(u64, u64), u64>> = boxes
        .par_iter()
        .map(|(bot, top)| {
            let mut local: HashMap<(u64, u64), u64> = HashMap::new();
            let mut f = bot.clone();
            f[0] = 0;
            loop {
                let t = bij::pi(g, &eids, &f);
                let mut s_mask = 0u64;
                let mut u_enc = 0u64;
                for v in 1..n {
                    let p = t[v] as usize;
                    if v ^ p == 1 {
                        s_mask |= 1 << (v >> 1);
                    } else {
                        let e = be_id[v >> 1][p >> 1];
                        u_enc += 1 << (2 * e);
                    }
                }
                *local.entry((s_mask, u_enc)).or_default() += 1;
                let mut i = 1;
                loop {
                    if i == n {
                        return local;
                    }
                    f[i] += 1;
                    if f[i] <= top[i] {
                        break;
                    }
                    f[i] = bot[i];
                    i += 1;
                }
            }
        })
        .collect();
    let mut classes: HashMap<(u64, u64), u64> = HashMap::new();
    for m in maps {
        for (k, v) in m {
            *classes.entry(k).or_default() += v;
        }
    }
    println!("{} (S,U) classes over the spanning trees of Q{dim}", classes.len());

    // base spanning trees as edge masks
    let base_eids = bij::edge_ids(&base);
    let mut base_tree_masks: Vec<u64> = Vec::new();
    {
        let mut bboxes: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
        burn::enumerate(&base, &mut |b| bboxes.push((b.bottom.to_vec(), b.top.to_vec())));
        for (bot, top) in bboxes {
            let mut f = bot.clone();
            f[0] = 0;
            loop {
                let t = bij::pi(&base, &base_eids, &f);
                let mut mask = 0u64;
                for v in 1..bn {
                    mask |= 1 << be_id[v][t[v] as usize];
                }
                base_tree_masks.push(mask);
                let mut i = 1;
                loop {
                    if i == bn {
                        break;
                    }
                    f[i] += 1;
                    if f[i] <= top[i] {
                        break;
                    }
                    f[i] = bot[i];
                    i += 1;
                }
                if i == bn {
                    break;
                }
            }
        }
    }

    // S-rooted forest test for an edge mask
    let s_forest = |mask: u64, s: u64| -> bool {
        let mut parent: Vec<usize> = (0..bn).collect();
        fn find(p: &mut Vec<usize>, mut x: usize) -> usize {
            while p[x] != x {
                p[x] = p[p[x]];
                x = p[x];
            }
            x
        }
        for e in 0..ne {
            if mask >> e & 1 == 1 {
                let (a, b) = base_edges[e];
                let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                if ra == rb {
                    return false; // cycle
                }
                parent[ra] = rb;
            }
        }
        // each component exactly one S vertex
        let mut seen = vec![0u32; bn];
        for v in 0..bn {
            if s >> v & 1 == 1 {
                let r = find(&mut parent, v);
                seen[r] += 1;
            }
        }
        (0..bn).all(|v| {
            let r = find(&mut parent, v);
            seen[r] == 1
        })
    };

    // evaluate both laws on every class
    let rows: Vec<((u64, u64), u64)> = classes.into_iter().collect();
    let verdicts: Vec<(u64, u64, u64, u32, u64, bool, bool)> = rows
        .par_iter()
        .map(|&((s, u_enc), count)| {
            let mut m1 = 0u32;
            let mut mult = [0u8; 64];
            for e in 0..ne {
                let c = (u_enc >> (2 * e) & 3) as u8;
                mult[e] = c;
                if c == 1 {
                    m1 += 1;
                }
            }
            // d = # base trees T (edge mask, one copy each) contained in U with
            // U - T a simple S-rooted forest
            let mut d = 0u64;
            for &tm in &base_tree_masks {
                let mut ok = true;
                let mut rem = 0u64;
                for e in 0..ne {
                    let used = (tm >> e & 1) as u8;
                    if used > mult[e] {
                        ok = false;
                        break;
                    }
                    let left = mult[e] - used;
                    if left > 1 {
                        ok = false;
                        break;
                    }
                    if left == 1 {
                        rem |= 1 << e;
                    }
                }
                if ok && s_forest(rem, s) {
                    d += 1;
                }
            }
            let mass = d > 0 && count * d == 1u64 << m1;
            let dicho = d > 0 && (count == d << m1 || 4 * count == d << m1);
            (s, u_enc, count, m1, d, mass, dicho)
        })
        .collect();

    let total = verdicts.len();
    let mass_ok = verdicts.iter().filter(|v| v.5).count();
    let dicho_ok = verdicts.iter().filter(|v| v.6).count();
    let violators = verdicts.iter().filter(|v| !v.5 && !v.6).count();
    println!("law  N*d = 2^m           holds in {mass_ok}/{total} classes");
    println!("law  N in {{d*2^m, d*2^(m-2)}} holds in {dicho_ok}/{total} classes");
    if violators > 0 {
        println!("classes violating BOTH: {violators}; samples (S U N d m):");
        for v in verdicts.iter().filter(|v| !v.5 && !v.6).take(8) {
            println!("  S={:#b} U={:#x} N={} d={} m={}", v.0, v.1, v.2, v.4, v.3);
        }
    }
    if let Some(path) = std::env::args().nth(3) {
        let mut w = std::io::BufWriter::new(std::fs::File::create(&path).unwrap());
        for v in verdicts.iter().filter(|v| !v.5 && !v.6) {
            writeln!(w, "{} {} {} {} {}", v.0, v.1, v.2, v.4, v.3).unwrap();
        }
        w.flush().unwrap();
        eprintln!("violators dumped to {path}");
    }
    println!("elapsed: {:.2?}", t0.elapsed());
}

/// Piece-2 data: exact census of (tree, root) pairs of Q_dim by vertical
/// support S in direction 0 (all directions are equivalent by symmetry;
/// computed for every direction to confirm). Reports count(S) and
/// count(S)/2^|S| — the residual N(S) a bijective recursion must explain.
fn cmd_census(g: &Graph, dim: u32) {
    use rayon::prelude::*;
    use std::collections::HashMap;
    let t0 = Instant::now();
    let n = g.n;
    let eids = bij::edge_ids(g);

    // parallel over the canonical boxes: each worker expands its boxes' PFs,
    // maps to trees, tallies (direction, support) over all n rootings
    let mut boxes: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
    burn::enumerate(g, &mut |b| boxes.push((b.bottom.to_vec(), b.top.to_vec())));
    let maps: Vec<HashMap<(u32, u64), u64>> = boxes
        .par_iter()
        .map(|(bot, top)| {
            let mut local: HashMap<(u32, u64), u64> = HashMap::new();
            let mut f = bot.clone();
            f[0] = 0;
            loop {
                let t = bij::pi(g, &eids, &f);
                // vertical support is root-independent: tally once, weight n (rootings)
                for d in 0..dim {
                    let mut s_mask = 0u64;
                    for u in 0..n {
                        if u >> d & 1 == 0 {
                            let b = u | (1usize << d);
                            if t[u] == b as i32 || t[b] == u as i32 {
                                s_mask |= 1 << u;
                            }
                        }
                    }
                    *local.entry((d, s_mask)).or_default() += n as u64;
                }
                let mut i = 1;
                loop {
                    if i == n {
                        return local;
                    }
                    f[i] += 1;
                    if f[i] <= top[i] {
                        break;
                    }
                    f[i] = bot[i];
                    i += 1;
                }
            }
        })
        .collect();
    let mut counts: HashMap<(u32, u64), u64> = HashMap::new();
    for m in maps {
        for (k, v) in m {
            *counts.entry(k).or_default() += v;
        }
    }

    // S-rooted spanning forest count of the base hypercube Q_{dim-1}:
    // determinant of the base Laplacian with the rows/columns of S removed
    // (all-minors Matrix-Tree theorem). Base vertex u <-> Q_dim vertex with
    // direction-0 bit 0, compressed label u (0..2^{dim-1}).
    let bn = 1usize << (dim - 1);
    let forest_det = |s: u64| -> u128 {
        let keep: Vec<usize> = (0..bn).filter(|&u| s >> (2 * u) & 1 == 0).collect();
        let m = keep.len();
        if m == 0 {
            return 1;
        }
        let mut a = vec![vec![0i128; m]; m];
        for (i, &u) in keep.iter().enumerate() {
            a[i][i] = (dim - 1) as i128;
            for (j, &v) in keep.iter().enumerate() {
                if (u ^ v).count_ones() == 1 {
                    a[i][j] = -1;
                }
            }
        }
        // Bareiss
        let mut sign = 1i128;
        let mut prev = 1i128;
        for k in 0..m {
            if a[k][k] == 0 {
                let Some(p) = (k + 1..m).find(|&i| a[i][k] != 0) else { return 0 };
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
    };
    let base_rooted: u128 = exact::stanley_qn_trees(dim - 1) * (1u128 << (dim - 1));

    // symmetry check across directions: compare multisets of counts by |S|
    let mut rows: Vec<(u32, u64, u64)> = counts
        .iter()
        .filter(|((d, _), _)| *d == 0)
        .map(|((_, s), &c)| (s.count_ones(), *s, c))
        .collect();
    rows.sort();
    println!("(tree,root) pairs of Q{dim} by direction-0 vertical support S (base = Q{}):", dim - 1);
    println!("conjecture: count(S) = 2^|S| * F_base(S) * {base_rooted}   [F = S-rooted forests of base]");
    println!("{:>4} {:>18} {:>14} {:>10} {:>10}", "|S|", "S (base mask)", "count", "F_base(S)", "match?");
    let mut by_size: HashMap<u32, Vec<u64>> = HashMap::new();
    let mut all_match = true;
    for (k, s, c) in &rows {
        let f = forest_det(*s);
        let predicted = (1u128 << k) * f * base_rooted;
        let matches = predicted == *c as u128;
        all_match &= matches;
        println!(
            "{:>4} {:>18} {:>14} {:>10} {:>10}",
            k,
            format!("{s:#b}"),
            c,
            f,
            if matches { "YES" } else { "NO" }
        );
        by_size.entry(*k).or_default().push(*c);
    }
    println!("support product conjecture holds for ALL classes: {all_match}");
    let total: u64 = rows.iter().map(|(_, _, c)| *c).sum();
    println!("total (tree,root) pairs: {total} (should be {} * 2^{dim})", exact::stanley_qn_trees(dim));
    // does count depend on S only through |S|?
    let mut keys: Vec<&u32> = by_size.keys().collect();
    keys.sort();
    for k in keys {
        let v = &by_size[k];
        let same = v.iter().all(|&x| x == v[0]);
        println!(
            "  |S| = {k}: {} classes, counts {}",
            v.len(),
            if same { format!("ALL EQUAL = {}", v[0]) } else { format!("VARY: {:?}", { let mut w = v.clone(); w.sort(); w }) }
        );
    }
    // direction symmetry
    for d in 1..dim {
        let mut other: Vec<u64> = counts.iter().filter(|((dd, _), _)| *dd == d).map(|(_, &c)| c).collect();
        let mut base: Vec<u64> = rows.iter().map(|(_, _, c)| *c).collect();
        other.sort();
        base.sort();
        println!("  direction {d} count-multiset == direction 0: {}", other == base);
    }
    println!("elapsed: {:.2?}", t0.elapsed());
}

/// Direction-1 experiment: transport parking functions to spanning trees via
/// the BCT bijection pi, read off the Bernardi spins of the out-edges at the
/// weight->=2 vertices, and test whether those 2^n - n - 1 bits are jointly
/// uniform (the shape a bijective proof of the tree formula needs).
/// samples = 0: exhaustive over all parking functions (n <= 4).
/// samples > 0: Wilson-sampled uniform spanning trees (any n): spins read
/// directly from the tree, plus mu roundtrip sanity.
fn cmd_spins(g: &Graph, dim: u32, samples: u64) {
    use std::collections::HashMap;
    let t0 = Instant::now();
    let eids = bij::edge_ids(g);
    let n = g.n;

    // For each direction d and each tree: S = set of base vertices (d-th bit 0)
    // whose vertical edge {u, u+e_d} is in the tree; spin at u = d-coordinate
    // of the PARENT endpoint of that edge (the side nearer the root).
    // Bernardi's Thm 1 transported: conditioned on S, spins should be iid uniform.
    let mut classes: HashMap<(u32, u64), HashMap<u64, u64>> = HashMap::new();
    let mut classes_free: HashMap<(u32, u64), HashMap<u64, u64>> = HashMap::new();
    let mut total = 0u64;

    fn reroot(parent: &[i32], r: usize) -> Vec<i32> {
        let mut p = parent.to_vec();
        // reverse the pointers along the old-root path from r
        let mut v = r as i32;
        let mut prev = -1i32;
        while v >= 0 {
            let next = p[v as usize];
            p[v as usize] = prev;
            prev = v;
            v = next;
        }
        p
    }

    fn tally(
        dim: u32,
        n: usize,
        parent: &[i32],
        into: &mut HashMap<(u32, u64), HashMap<u64, u64>>,
    ) {
        for d in 0..dim {
            let mut s_mask = 0u64;
            let mut spinvec = 0u64;
            for u in 0..n {
                if u >> d & 1 == 1 {
                    continue;
                }
                let b = u | (1usize << d);
                if parent[u] == b as i32 {
                    s_mask |= 1 << u;
                    spinvec |= 1 << u; // parent endpoint b has d-coordinate 1
                } else if parent[b] == u as i32 {
                    s_mask |= 1 << u; // parent endpoint u has d-coordinate 0
                }
            }
            *into.entry((d, s_mask)).or_default().entry(spinvec).or_default() += 1;
        }
    }

    let mut record = |parent: &[i32]| {
        total += 1;
        tally(dim, n, parent, &mut classes);
        for r in 0..n {
            let rp = reroot(parent, r);
            tally(dim, n, &rp, &mut classes_free);
        }
    };

    if samples == 0 {
        let mut boxes: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
        burn::enumerate(g, &mut |b| boxes.push((b.bottom.to_vec(), b.top.to_vec())));
        for (bot, top) in boxes {
            let mut f = bot.clone();
            f[0] = 0;
            loop {
                let t = bij::pi(g, &eids, &f);
                record(&t);
                let mut i = 1;
                loop {
                    if i == n {
                        break;
                    }
                    f[i] += 1;
                    if f[i] <= top[i] {
                        break;
                    }
                    f[i] = bot[i];
                    i += 1;
                }
                if i == n {
                    break;
                }
            }
        }
        println!("exhaustive: {total} parking functions -> trees via BCT pi");
    } else {
        // independent samples: rooted-at-0 tally from each Wilson tree, plus a
        // single uniformly random re-rooting (one entry per tree keeps the
        // chi-square calibration valid; re-rooting at ALL vertices would put
        // correlated entries in one class and inflate the statistic)
        fn next_rand(x: &mut u64) -> u64 {
            *x = x.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = *x;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^ (z >> 31)
        }
        let mut rng = 0x7ade_2026_0819u64;
        for _ in 0..samples {
            let t = bij::wilson(g, &mut rng);
            total += 1;
            tally(dim, n, &t, &mut classes);
            let r = (next_rand(&mut rng) % n as u64) as usize;
            let rp = reroot(&t, r);
            tally(dim, n, &rp, &mut classes_free);
        }
        println!("{total} Wilson-sampled uniform spanning trees (one random re-rooting each)");
    }

    // Evaluate uniformity within every (direction, support) class
    let report = |label: &str, classes: &HashMap<(u32, u64), HashMap<u64, u64>>| {
        println!("{label}:");
        for d in 0..dim {
            let mut nclasses = 0u64;
            let mut exact_uniform = 0u64;
            let mut chi2 = 0f64;
            let mut dof = 0u64;
            let mut worst: (f64, u64) = (0.0, 0);
            for ((dd, s), cells) in classes.iter() {
                if *dd != d {
                    continue;
                }
                nclasses += 1;
                let k = s.count_ones();
                let class_total: u64 = cells.values().sum();
                let ncells = 1u64 << k;
                let expected = class_total as f64 / ncells as f64;
                let mut class_chi2 = 0f64;
                let mut all_equal = true;
                let mut seen = 0u64;
                for (_, &c) in cells.iter() {
                    let dev = c as f64 - expected;
                    class_chi2 += dev * dev / expected;
                    if (c as f64 - expected).abs() > 1e-9 {
                        all_equal = false;
                    }
                    seen += 1;
                }
                // cells never hit contribute their expectation
                class_chi2 += (ncells - seen) as f64 * expected;
                if seen != ncells {
                    all_equal = false;
                }
                if all_equal {
                    exact_uniform += 1;
                }
                chi2 += class_chi2;
                dof += ncells - 1;
                if class_chi2 > worst.0 {
                    worst = (class_chi2, *s);
                }
            }
            println!(
                "  direction {d}: {nclasses} support-classes, {exact_uniform} EXACTLY uniform; \
                 total chi2 = {chi2:.1} on {dof} dof (worst class chi2 {:.1} at S={:#b})",
                worst.0, worst.1
            );
        }
    };
    report("vertical-edge spins, trees ROOTED AT 0 (via BCT pi)", &classes);
    report("vertical-edge spins, same trees re-rooted at ALL vertices", &classes_free);
    println!("elapsed: {:.2?}", t0.elapsed());
}

fn cmd_hvector(g: &Graph) {
    let t0 = Instant::now();
    let agg = burn::aggregate(g);
    let gd = agg.by_degree.len() - 1;
    println!("parking functions of G by total degree (root excluded):");
    for (d, c) in agg.by_degree.iter().enumerate() {
        println!("  deg {d:>3}: {c}");
    }
    let total: u128 = agg.by_degree.iter().sum();
    println!("total = {total} (spanning trees), top = {} (maximal PFs)", agg.by_degree[gd]);
    let coeffs: Vec<String> = agg.by_degree.iter().rev().map(|c| c.to_string()).collect();
    println!("T_G(1,y) = {}", coeffs
        .iter()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.clone() } else { format!("{c}*y^{i}") })
        .collect::<Vec<_>>()
        .join(" + "));
    println!("elapsed: {:.2?}", t0.elapsed());
}

fn writer_for(path: Option<&String>) -> Box<dyn std::io::Write> {
    match path {
        Some(p) => Box::new(std::io::BufWriter::new(
            std::fs::File::create(p).unwrap_or_else(|e| panic!("cannot create {p}: {e}")),
        )),
        None => Box::new(std::io::BufWriter::new(std::io::stdout())),
    }
}

fn cmd_list_max(g: &Graph, out: Option<&String>) {
    let mut w = writer_for(out);
    let mut count = 0u64;
    burn::enumerate(g, &mut |b| {
        writeln!(w, "{}", fmt_pf(b.top)).unwrap();
        count += 1;
    });
    w.flush().unwrap();
    eprintln!("{count} maximal parking functions");
}

fn cmd_boxes(g: &Graph, out: Option<&String>) {
    let mut w = writer_for(out);
    let mut count = 0u64;
    let mut volume: u128 = 0;
    burn::enumerate(g, &mut |b| {
        let vol: u128 = (1..g.n).map(|v| (b.top[v] - b.bottom[v] + 1) as u128).product();
        writeln!(
            w,
            "order {:?}  bottom {}  top {}  vol {}",
            &b.order[1..],
            fmt_pf(b.bottom),
            fmt_pf(b.top),
            vol
        )
        .unwrap();
        count += 1;
        volume += vol;
    });
    w.flush().unwrap();
    eprintln!("{count} boxes partitioning {volume} parking functions");
}

fn ok(b: bool) -> &'static str {
    if b {
        "[ok]"
    } else {
        "[MISMATCH]"
    }
}

fn cmd_analyze(g: &Graph, dim: Option<u32>, what: &str) {
    let all = what == "all";

    if all || what == "snf" {
        println!("== sandpile group (Smith normal form of reduced Laplacian) ==");
        let t0 = Instant::now();
        let sp = analyze::sandpile_group(g);
        let inv = sp.invariants();
        println!("invariant factors     : {inv:?}");
        println!(
            "group structure       : {}",
            inv.iter().map(|d| format!("Z_{d}")).collect::<Vec<_>>().join(" x ")
        );
        println!("group order           : {} {}", sp.order(), ok(sp.order() == exact::kirchhoff(g)));
        println!("2-rank (even factors) : {}", inv.iter().filter(|&&d| d % 2 == 0).count());
        // coset-representative check: distinct maximal PFs must land in distinct classes
        if g.n <= 16 {
            let mut keys: Vec<u128> = Vec::new();
            burn::enumerate(g, &mut |b| keys.push(sp.class_key(&b.top[1..])));
            let total = keys.len();
            keys.sort_unstable();
            keys.dedup();
            println!("maximal PF classes    : {} distinct of {} {}", keys.len(), total, ok(keys.len() == total));
        } else {
            println!("maximal PF class check: skipped (too many to enumerate)");
        }
        println!("elapsed: {:.2?}\n", t0.elapsed());
    }

    if all || what == "mobius" {
        let bound: u128 = (1..g.n).map(|v| g.weighted_degree(v) as u128).product();
        if bound > 200_000_000 {
            println!("== mobius == skipped (brute-force space {bound} too large)\n");
        } else {
            println!("== Mobius function of the parking-function poset ==");
            let t0 = Instant::now();
            let r = analyze::mobius_experiment(g);
            println!("parking functions     : {}", r.total);
            println!("maximal (degree g)    : {}", r.maximal);
            println!("nonzero mu(x, 1hat)   : {} of {}", r.nonzero_mu, r.total);
            println!(
                "crosscut identity     : -sum mu*vol = {} {}",
                r.identity_lhs,
                ok(r.identity_lhs == r.total as i128)
            );
            println!("BCT Lemma 4.12        : {}", ok(r.lemma_412_holds));
            let mut rows: Vec<((u32, i128), usize)> = r.by_degree.into_iter().collect();
            rows.sort();
            println!("degree | mu value | count");
            for ((d, m), c) in rows {
                println!("  {d:>4} | {m:>8} | {c}");
            }
            println!("elapsed: {:.2?}\n", t0.elapsed());
        }
    }

    if all || what == "orbits" {
        match dim {
            Some(d) if d <= 4 => {
                println!("== orbit census under coordinate permutations (S_{d}) ==");
                let t0 = Instant::now();
                let mut packed: Vec<u64> = Vec::new();
                burn::enumerate(g, &mut |b| packed.push(analyze::pack(&b.top[1..])));
                let r = analyze::orbit_census(d, &packed);
                println!("maximal PFs           : {}", r.items);
                println!("orbits                : {}", r.orbits);
                println!("orbit sizes (size x count): {:?}", r.size_histogram);
                println!("elapsed: {:.2?}\n", t0.elapsed());
            }
            _ => println!("== orbits == only implemented for hypercubes q1..q4\n"),
        }
    }

    if all || what == "boxmap" {
        match dim {
            Some(d) => {
                println!("== BCT canonical box dom(f^n), f^n(v)=wgt(v)-1, inside the sandpile group ==");
                let t0 = Instant::now();
                let sp = analyze::sandpile_group(g);
                let r = analyze::boxmap_experiment(g, d, &sp);
                println!("box volume            : {}", r.box_volume);
                println!(
                    "distinct classes      : {} {}",
                    r.distinct_classes,
                    ok(r.distinct_classes as u128 == r.box_volume)
                );
                println!("contains identity     : {}", r.contains_zero);
                println!(
                    "closed under addition : {}",
                    match r.is_subgroup {
                        Some(b) => b.to_string(),
                        None => "check skipped (too large)".into(),
                    }
                );
                println!("group order           : {}", r.group_order);
                println!("group 2-rank          : {}", r.two_rank);
                println!("elapsed: {:.2?}\n", t0.elapsed());
            }
            None => println!("== boxmap == only defined for hypercubes\n"),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("");
    if cmd == "validate" {
        validate();
        return;
    }
    let Some(spec) = args.get(1) else {
        eprintln!("{USAGE}");
        std::process::exit(2);
    };
    let (g, dim) = match parse_graph(spec) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    match cmd {
        "count" => cmd_count(&g, dim),
        "estimate" => {
            let samples: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1_000_000);
            cmd_estimate(&g, dim, samples);
        }
        "hvector" => cmd_hvector(&g),
        "list-max" => cmd_list_max(&g, args.get(2)),
        "boxes" => cmd_boxes(&g, args.get(2)),
        "analyze" => cmd_analyze(&g, dim, args.get(2).map(String::as_str).unwrap_or("all")),
        "trees" => {
            let eids = bij::edge_ids(&g);
            let mut w = writer_for(args.get(2));
            let mut boxes: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
            burn::enumerate(&g, &mut |b| boxes.push((b.bottom.to_vec(), b.top.to_vec())));
            for (bot, top) in boxes {
                let mut f = bot.clone();
                f[0] = 0;
                loop {
                    let t = bij::pi(&g, &eids, &f);
                    writeln!(w, "{:?}", &t[..]).unwrap();
                    let mut i = 1;
                    loop {
                        if i == g.n {
                            break;
                        }
                        f[i] += 1;
                        if f[i] <= top[i] {
                            break;
                        }
                        f[i] = bot[i];
                        i += 1;
                    }
                    if i == g.n {
                        break;
                    }
                }
            }
            w.flush().unwrap();
        }
        "project" => {
            let Some(d) = dim else {
                eprintln!("project is defined for hypercubes only");
                std::process::exit(1);
            };
            cmd_project(&g, d);
        }
        "census" => {
            let Some(d) = dim else {
                eprintln!("census is defined for hypercubes only");
                std::process::exit(1);
            };
            cmd_census(&g, d);
        }
        "spins" => {
            let Some(d) = dim else {
                eprintln!("spins is defined for hypercubes only");
                std::process::exit(1);
            };
            let samples: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            cmd_spins(&g, d, samples);
        }
        _ => {
            eprintln!("{USAGE}");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// validation: the canonical enumerator against brute force
// ---------------------------------------------------------------------------

fn expand_boxes(g: &Graph) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut all = Vec::new();
    let mut tops = Vec::new();
    burn::enumerate(g, &mut |b| {
        tops.push(b.top[1..].to_vec());
        let n1 = g.n - 1;
        let mut f: Vec<u32> = b.bottom[1..].to_vec();
        loop {
            all.push(f.clone());
            let mut i = 0;
            loop {
                if i == n1 {
                    return;
                }
                f[i] += 1;
                if f[i] <= b.top[i + 1] {
                    break;
                }
                f[i] = b.bottom[i + 1];
                i += 1;
            }
        }
    });
    (all, tops)
}

fn check_graph(g: &Graph, label: &str) {
    let (mut all, mut tops) = expand_boxes(g);
    let dup = all.len();
    all.sort();
    all.dedup();
    assert_eq!(all.len(), dup, "{label}: boxes overlap");

    let mut brute = dhar::brute_all(g);
    brute.sort();
    assert_eq!(all, brute, "{label}: box union != parking function set");

    let mut brute_max = dhar::maximal_of(&brute);
    brute_max.sort();
    tops.sort();
    assert_eq!(tops, brute_max, "{label}: box tops != maximal parking functions");

    let genus = g.genus() as u32;
    assert!(
        tops.iter().all(|t| t.iter().sum::<u32>() == genus),
        "{label}: a maximal PF does not have degree g"
    );

    let agg = burn::aggregate(g);
    assert_eq!(agg.leaves as usize, tops.len(), "{label}: leaf count");
    assert_eq!(agg.trees, brute.len() as u128, "{label}: tree count vs brute PF count");
    assert_eq!(agg.trees, exact::kirchhoff(g), "{label}: tree count vs Matrix-Tree");
    let total: u128 = agg.by_degree.iter().sum();
    assert_eq!(total, agg.trees, "{label}: degree histogram total");
    assert_eq!(*agg.by_degree.last().unwrap(), agg.leaves as u128, "{label}: top degree count");
    println!("  {label}: ok ({} PFs, {} maximal)", brute.len(), tops.len());
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, m: u32) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 33) % m as u64) as u32
    }
}

fn random_connected(rng: &mut Lcg, n: usize) -> Graph {
    loop {
        let mut adj = vec![vec![0u32; n]; n];
        for i in 0..n {
            for j in i + 1..n {
                let m = if rng.next(3) == 0 { rng.next(4) } else { rng.next(2) };
                adj[i][j] = m;
                adj[j][i] = m;
            }
        }
        if let Ok(g) = Graph::from_matrix(adj) {
            return g;
        }
    }
}

fn validate() {
    println!("hypercubes:");
    for d in 1..=3 {
        check_graph(&Graph::hypercube(d), &format!("q{d}"));
    }

    println!("draft repo sample multigraph:");
    let sample = Graph::from_matrix(vec![
        vec![0, 1, 0, 2],
        vec![1, 0, 1, 1],
        vec![0, 1, 0, 3],
        vec![2, 1, 3, 0],
    ])
    .unwrap();
    check_graph(&sample, "sampleMatrix");
    let (_, mut tops) = expand_boxes(&sample);
    tops.sort();
    let mut expected = vec![vec![0, 0, 5], vec![0, 3, 2], vec![1, 3, 1], vec![2, 2, 1]];
    expected.sort();
    assert_eq!(tops, expected, "sampleMatrix: does not match the 2016 Python results");
    println!("  matches the maximal PFs recorded in QnMaxParkingFunctionFinder.py");

    println!("BCT Example 5.1 (exotic maximal Q3-parking function):");
    let (_, q3tops) = expand_boxes(&Graph::hypercube(3));
    assert!(q3tops.contains(&vec![1, 0, 0, 2, 0, 0, 2]), "exotic example missing");
    println!("  [-1,1,0,0,2,0,0,2] found among the 133");

    println!("random connected multigraphs:");
    let mut rng = Lcg(20260819);
    for i in 0..40 {
        let n = 3 + (i % 4) as usize;
        let g = random_connected(&mut rng, n);
        check_graph(&g, &format!("random#{i} (n={n})"));
    }
    println!("all checks passed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q2_q3_against_brute_force() {
        for d in 1..=3 {
            check_graph(&Graph::hypercube(d), &format!("q{d}"));
        }
    }

    #[test]
    fn q3_known_counts() {
        let agg = burn::aggregate(&Graph::hypercube(3));
        assert_eq!(agg.leaves, 133);
        assert_eq!(agg.trees, 384);
    }

    #[test]
    fn draft_sample_multigraph() {
        let g = Graph::from_matrix(vec![
            vec![0, 1, 0, 2],
            vec![1, 0, 1, 1],
            vec![0, 1, 0, 3],
            vec![2, 1, 3, 0],
        ])
        .unwrap();
        check_graph(&g, "sampleMatrix");
        let (_, mut tops) = expand_boxes(&g);
        tops.sort();
        let mut expected = vec![vec![0, 0, 5], vec![0, 3, 2], vec![1, 3, 1], vec![2, 2, 1]];
        expected.sort();
        assert_eq!(tops, expected);
    }

    #[test]
    fn random_multigraphs() {
        let mut rng = Lcg(987654321);
        for i in 0..25 {
            let n = 3 + (i % 4) as usize;
            let g = random_connected(&mut rng, n);
            check_graph(&g, &format!("random#{i}"));
        }
    }

    #[test]
    fn snf_q3_group() {
        let g = Graph::hypercube(3);
        let sp = analyze::sandpile_group(&g);
        assert_eq!(sp.order(), 384);
        // Bai Thm 1.1: K(Q_n) has exactly 2^(n-1) - 1 nontrivial invariant factors
        assert_eq!(sp.invariants().len(), 3);
        // parking functions are coset representatives: classes all distinct
        let all = dhar::brute_all(&g);
        let mut keys: Vec<u128> = all.iter().map(|f| sp.class_key(f)).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), 384);
    }

    #[test]
    fn mobius_q2_q3() {
        for d in 2..=3 {
            let g = Graph::hypercube(d);
            let r = analyze::mobius_experiment(&g);
            assert_eq!(r.identity_lhs, r.total as i128, "crosscut identity q{d}");
            assert!(r.lemma_412_holds, "Lemma 4.12 q{d}");
        }
    }

    #[test]
    fn orbits_q3() {
        let g = Graph::hypercube(3);
        let mut packed: Vec<u64> = Vec::new();
        burn::enumerate(&g, &mut |b| packed.push(analyze::pack(&b.top[1..])));
        let r = analyze::orbit_census(3, &packed);
        assert_eq!(r.items, 133);
        let total: usize = r.size_histogram.iter().map(|(s, c)| s * c).sum();
        assert_eq!(total, 133);
    }
}
