// for each elem i, its parent is parent[i]
#[derive(Default)]
pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    // on init, parent is the num itself
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    // worst O(N)
    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        self.parent[y] = x
    }

    // worst O(N)
    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}
