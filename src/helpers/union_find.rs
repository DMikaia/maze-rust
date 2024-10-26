pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        let rank = vec![0; size];
        for i in 0..size {
            parent[i] = i;
        }
        UnionFind { parent, rank }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }

    pub fn is_fully_connected(&mut self) -> bool {
        let root = self.find(0);
        for i in 1..self.parent.len() {
            if self.find(i) != root {
                return false;
            }
        }
        true
    }
}
