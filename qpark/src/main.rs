mod analyze;
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
        let mut keys: Vec<u128> = Vec::new();
        burn::enumerate(g, &mut |b| keys.push(sp.class_key(&b.top[1..])));
        let total = keys.len();
        keys.sort_unstable();
        keys.dedup();
        println!("maximal PF classes    : {} distinct of {} {}", keys.len(), total, ok(keys.len() == total));
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
        "hvector" => cmd_hvector(&g),
        "list-max" => cmd_list_max(&g, args.get(2)),
        "boxes" => cmd_boxes(&g, args.get(2)),
        "analyze" => cmd_analyze(&g, dim, args.get(2).map(String::as_str).unwrap_or("all")),
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
