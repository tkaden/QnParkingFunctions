/// Connected multigraph on at most 64 vertices. Vertex 0 is always the root q
/// of the parking-function definition.
#[derive(Clone)]
pub struct Graph {
    pub n: usize,
    /// symmetric multiplicity matrix, zero diagonal
    pub adj: Vec<Vec<u32>>,
    /// neighbor bitmasks (multiplicity ignored)
    pub nbr: Vec<u64>,
}

pub fn vertex_mask(n: usize) -> u64 {
    if n == 64 {
        !0
    } else {
        (1u64 << n) - 1
    }
}

impl Graph {
    pub fn from_matrix(adj: Vec<Vec<u32>>) -> Result<Graph, String> {
        let n = adj.len();
        if n < 2 {
            return Err("need at least 2 vertices".into());
        }
        if n > 64 {
            return Err("at most 64 vertices supported (bitmask representation)".into());
        }
        for i in 0..n {
            if adj[i].len() != n {
                return Err(format!("row {i} has length {} != {n}", adj[i].len()));
            }
            if adj[i][i] != 0 {
                return Err(format!("nonzero diagonal entry at vertex {i}"));
            }
            for j in 0..n {
                if adj[i][j] != adj[j][i] {
                    return Err(format!("matrix not symmetric at ({i},{j})"));
                }
            }
        }
        let nbr: Vec<u64> = (0..n)
            .map(|i| (0..n).filter(|&j| adj[i][j] > 0).fold(0u64, |m, j| m | 1 << j))
            .collect();
        let g = Graph { n, adj, nbr };
        if !g.connected() {
            return Err("graph is not connected".into());
        }
        Ok(g)
    }

    fn connected(&self) -> bool {
        let mut seen = 1u64;
        let mut stack = vec![0usize];
        while let Some(v) = stack.pop() {
            let mut m = self.nbr[v] & !seen;
            while m != 0 {
                let w = m.trailing_zeros() as usize;
                m &= m - 1;
                seen |= 1 << w;
                stack.push(w);
            }
        }
        seen.count_ones() as usize == self.n
    }

    /// Hypercube Q_dim: vertices are bitstrings, edges join Hamming-distance-1
    /// pairs. Root 0 is the all-zeros vertex.
    pub fn hypercube(dim: u32) -> Graph {
        assert!((1..=6).contains(&dim), "hypercube dimension must be 1..=6");
        let n = 1usize << dim;
        let mut adj = vec![vec![0u32; n]; n];
        for v in 0..n {
            for b in 0..dim {
                adj[v][v ^ (1 << b)] = 1;
            }
        }
        Graph::from_matrix(adj).unwrap()
    }

    pub fn weighted_degree(&self, v: usize) -> u32 {
        self.adj[v].iter().sum()
    }

    pub fn weighted_edges(&self) -> u64 {
        (0..self.n).map(|v| self.weighted_degree(v) as u64).sum::<u64>() / 2
    }

    /// Cyclomatic number g = |E| - |V| + 1. Every maximal parking function has
    /// total degree g (BCT Prop 2.3 + Cor 3.2).
    pub fn genus(&self) -> u64 {
        self.weighted_edges() - self.n as u64 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hypercube_basics() {
        let q3 = Graph::hypercube(3);
        assert_eq!(q3.n, 8);
        assert!((0..8).all(|v| q3.weighted_degree(v) == 3));
        assert_eq!(q3.weighted_edges(), 12);
        assert_eq!(q3.genus(), 5);
    }

    #[test]
    fn rejects_disconnected() {
        let m = vec![vec![0, 1, 0, 0], vec![1, 0, 0, 0], vec![0, 0, 0, 1], vec![0, 0, 1, 0]];
        assert!(Graph::from_matrix(m).is_err());
    }
}
