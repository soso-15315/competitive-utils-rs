#![allow(dead_code)]

fn main() {}

// UnionFind Tree
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }

        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);

        if x == y {
            return;
        }

        if self.siz[x] < self.siz[y] {
            self.par[x] = y;
            self.siz[y] += self.siz[x];
        } else {
            self.par[y] = x;
            self.siz[x] += self.siz[y];
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.siz[x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(100);

        assert_eq!(uf.is_same(0, 1), false);
        assert_eq!(uf.is_same(0, 2), false);
        assert_eq!(uf.is_same(0, 3), false);

        uf.unite(0, 1);
        uf.unite(1, 3);

        assert_eq!(uf.is_same(0, 1), true);
        assert_eq!(uf.is_same(0, 2), false);
        assert_eq!(uf.is_same(0, 3), true);
    }
}
