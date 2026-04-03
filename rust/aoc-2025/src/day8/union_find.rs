pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<u64>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        if self.par[i] != i {
            self.par[i] = self.root(self.par[i])
        }

        self.par[i]
    }

    pub fn merge(&mut self, a: usize, b: usize) {
        if self.is_same_component(a, b) {
            return;
        }
        let a_root = self.root(a);
        let b_root = self.root(b);
        // TODO: do proper union by size
        self.size[b_root] += self.size[a_root];
        self.par[a_root] = b_root;
    }

    pub fn is_same_component(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    pub fn sizes(&self) -> Vec<u64> {
        let mut result = Vec::new();

        for (idx, i) in self.par.iter().enumerate() {
            if idx == *i {
                result.push(self.size[idx]);
            }
        }

        result
    }
}
