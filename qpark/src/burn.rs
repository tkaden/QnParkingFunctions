use crate::graph::{vertex_mask, Graph};
use rayon::prelude::*;

/// Canonical enumeration of G-parking functions, grouped into boxes.
///
/// Every parking function f (root q = 0, Dhar convention: f is superstable
/// iff repeatedly burning any vertex v with f(v) < (edge-multiplicity-weighted
/// count of burnt neighbors), starting from q burnt, burns everything) has a
/// unique LEX-MINIMAL burn order sigma(f): at each step burn the smallest
/// burnable vertex. This search enumerates exactly the orders arising as
/// sigma(f) for some f, once each, WITHOUT constructing f up front:
///
///   * Burnability of the k-th burned vertex v requires f(v) < wcnt_k(v),
///     where wcnt_k is v's burnt-neighbor weight at that moment.
///   * Lex-minimality requires every unburnt u < v to NOT be burnable at
///     step k, i.e. f(u) >= wcnt_k(u). Since wcnt is nondecreasing along the
///     order, the binding constraint is the LAST such skip: keep a watermark
///     w2(u) = wcnt(u) at the most recent step that burned some v > u.
///
/// So at the step burning v, the compatible values are exactly the interval
/// f(v) in [w2(v), wcnt(v) - 1], nonempty precisely when wcnt(v) > w2(v),
/// which is the branch condition. Consequences (each verified in tests):
///
///   * each leaf is the box {bottom <= f <= top} of parking functions whose
///     lex-min burn order is that leaf's order (bottom = w2 at burn time,
///     top = wcnt - 1 at burn time);
///   * the boxes PARTITION the set of all parking functions, so
///     sum of box volumes = number of spanning trees T_G(1,1);
///   * every box top has total degree g = |E| - |V| + 1, tops are pairwise
///     distinct, and they are exactly the maximal parking functions, so
///     leaf count = T_G(1,0).
pub struct Boxes<'a> {
    pub order: &'a [u32],
    /// bottom[v], top[v] indexed by vertex; entries for the root are unused
    pub bottom: &'a [u32],
    pub top: &'a [u32],
}

#[derive(Clone)]
pub struct State {
    burnt: u64,
    order: Vec<u32>,
    wcnt: Vec<u32>,
    w2: Vec<u32>,
    bottom: Vec<u32>,
    top: Vec<u32>,
}

impl State {
    pub fn root(g: &Graph) -> State {
        let n = g.n;
        let mut s = State {
            burnt: 1,
            order: vec![0],
            wcnt: vec![0; n],
            w2: vec![0; n],
            bottom: vec![0; n],
            top: vec![0; n],
        };
        for w in 1..n {
            s.wcnt[w] = g.adj[0][w];
        }
        s
    }

    pub fn depth(&self) -> usize {
        self.order.len()
    }
}

fn apply_burn(g: &Graph, s: &mut State, v: usize) -> Vec<(usize, u32)> {
    s.bottom[v] = s.w2[v];
    s.top[v] = s.wcnt[v] - 1;
    // burning v commits every smaller unburnt vertex to stay unburnable now
    let mut undo = Vec::new();
    for u in 1..v {
        if s.burnt >> u & 1 == 0 && s.w2[u] != s.wcnt[u] {
            undo.push((u, s.w2[u]));
            s.w2[u] = s.wcnt[u];
        }
    }
    s.burnt |= 1 << v;
    s.order.push(v as u32);
    let mut m = g.nbr[v] & !s.burnt;
    while m != 0 {
        let w = m.trailing_zeros() as usize;
        m &= m - 1;
        s.wcnt[w] += g.adj[v][w];
    }
    undo
}

fn undo_burn(g: &Graph, s: &mut State, v: usize, undo: Vec<(usize, u32)>) {
    let mut m = g.nbr[v] & !s.burnt;
    while m != 0 {
        let w = m.trailing_zeros() as usize;
        m &= m - 1;
        s.wcnt[w] -= g.adj[v][w];
    }
    s.order.pop();
    s.burnt &= !(1 << v);
    for (u, old) in undo {
        s.w2[u] = old;
    }
}

/// A vertex with no unburnt neighbors can never raise its wcnt again; if its
/// watermark already equals wcnt it can never be burned, so the branch is dead.
fn dead_end(g: &Graph, s: &State) -> bool {
    let mut m = !s.burnt & vertex_mask(g.n);
    while m != 0 {
        let u = m.trailing_zeros() as usize;
        m &= m - 1;
        if s.wcnt[u] <= s.w2[u] && g.nbr[u] & !s.burnt == 0 {
            return true;
        }
    }
    false
}

pub fn dfs<F: FnMut(Boxes)>(g: &Graph, s: &mut State, cb: &mut F) {
    if s.depth() == g.n {
        cb(Boxes { order: &s.order, bottom: &s.bottom, top: &s.top });
        return;
    }
    for v in 1..g.n {
        if s.burnt >> v & 1 == 0 && s.wcnt[v] > s.w2[v] {
            let undo = apply_burn(g, s, v);
            if !dead_end(g, s) {
                dfs(g, s, cb);
            }
            undo_burn(g, s, v, undo);
        }
    }
}

pub fn enumerate<F: FnMut(Boxes)>(g: &Graph, cb: &mut F) {
    let mut s = State::root(g);
    dfs(g, &mut s, cb);
}

/// Collect search states at the given depth (or leaves reached earlier), for
/// parallel processing of the subtrees.
fn frontier(g: &Graph, depth: usize) -> Vec<State> {
    fn collect(g: &Graph, s: &mut State, depth: usize, out: &mut Vec<State>) {
        if s.depth() >= depth || s.depth() == g.n {
            out.push(s.clone());
            return;
        }
        for v in 1..g.n {
            if s.burnt >> v & 1 == 0 && s.wcnt[v] > s.w2[v] {
                let undo = apply_burn(g, s, v);
                if !dead_end(g, s) {
                    collect(g, s, depth, out);
                }
                undo_burn(g, s, v, undo);
            }
        }
    }
    let mut out = Vec::new();
    let mut s = State::root(g);
    collect(g, &mut s, depth, &mut out);
    out
}

/// Aggregates computed in one pass over all leaves.
#[derive(Clone)]
pub struct Agg {
    /// number of leaves = number of maximal parking functions = T_G(1,0)
    pub leaves: u64,
    /// sum of box volumes = number of parking functions = spanning trees
    pub trees: u128,
    /// parking functions counted by total degree 0..=g; reversing gives the
    /// coefficients of T_G(1,y) (Merino's theorem)
    pub by_degree: Vec<u128>,
    scratch: Vec<u128>,
}

impl Agg {
    fn new(genus: usize) -> Agg {
        Agg {
            leaves: 0,
            trees: 0,
            by_degree: vec![0; genus + 1],
            scratch: vec![0; genus + 1],
        }
    }

    fn add_leaf(&mut self, n: usize, b: &Boxes) {
        self.leaves += 1;
        let mut vol: u128 = 1;
        for v in 1..n {
            vol *= (b.top[v] - b.bottom[v] + 1) as u128;
        }
        self.trees += vol;
        // degree polynomial of the box: prod over v of (y^bottom + .. + y^top)
        let glen = self.by_degree.len();
        let mut poly = vec![0u128; glen];
        poly[0] = 1;
        let mut deg = 0usize;
        for v in 1..n {
            let (lo, hi) = (b.bottom[v] as usize, b.top[v] as usize);
            let next = &mut self.scratch;
            for d in next.iter_mut() {
                *d = 0;
            }
            for d in 0..=deg {
                if poly[d] != 0 {
                    for j in lo..=hi {
                        next[d + j] += poly[d];
                    }
                }
            }
            deg += hi;
            std::mem::swap(&mut poly, next);
        }
        for d in 0..glen {
            self.by_degree[d] += poly[d];
        }
    }

    fn merge(mut self, other: Agg) -> Agg {
        self.leaves += other.leaves;
        self.trees += other.trees;
        for (a, b) in self.by_degree.iter_mut().zip(other.by_degree.iter()) {
            *a += b;
        }
        self
    }
}

/// Parallel one-pass computation of (leaf count, tree count, degree histogram).
pub fn aggregate(g: &Graph) -> Agg {
    let genus = g.genus() as usize;
    let mut depth = 2;
    let mut states = frontier(g, depth);
    while states.len() < 512 && depth < g.n {
        depth += 1;
        states = frontier(g, depth);
    }
    states
        .into_par_iter()
        .map(|mut st| {
            let mut a = Agg::new(genus);
            let n = g.n;
            dfs(g, &mut st, &mut |b| a.add_leaf(n, &b));
            a
        })
        .reduce(|| Agg::new(genus), Agg::merge)
}
