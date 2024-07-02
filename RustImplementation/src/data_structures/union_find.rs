struct UnionFind {
    size: usize,
    sz: Vec<usize>,
    id: Vec<usize>,
    num_components: usize
}

impl UnionFind {
    fn new(size: usize) -> Result<UnionFind, &'static str> {
        if size == 0 {
            return Err("UnionFind with size <= 0 is not allowed");
        }

        Ok(Self {
            size,
            sz: vec![1; size],
            id: (0..size).collect(),
            num_components: size
        })
    }

    /// Find which component/set 'p' belongs to, takes amortized constant time
    fn find(&mut self, mut p: usize) -> usize {
        let mut root = p;

        // Find the root of the component/set
        while root != self.id[root] {
            root = self.id[root];
        }

        // Compress the path leading back to the root.
        // Doing this operation is called "path compression"
        // and is what gives us amortized constant time complexity
        while p != root {
            let next = self.id[p];
            self.id[p] = root;
            p = next;
        }

        root
    }

    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    fn component_size(&mut self, p: usize) -> usize {
        let i = self.find(p);
        self.sz[i]
    }

    fn size(&self) -> usize {
        self.size
    }

    fn components(&self) -> usize {
        self.num_components
    }

    fn unify(&mut self, p: usize, q: usize) {
        let root_1 = self.find(p);
        let root_2 = self.find(q);

        if root_1 == root_2 {
            return;
        }

        if self.sz[root_1] < self.sz[root_2] {
            self.sz[root_2] += self.sz[root_1];
            self.id[root_1] = root_2;
        } else {
            self.sz[root_1] += self.sz[root_2];
            self.id[root_2] = root_1;
        }

        self.num_components -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn test_new() {
        let uf = UnionFind::new(10).unwrap();
        assert_eq!(uf.size(), 10);
        assert_eq!(uf.components(), 10);
    }

    #[test]
    #[should_panic(expected = "UnionFind with size <= 0 is not allowed")]
    fn test_new_with_invalid_size() {
        UnionFind::new(0).unwrap();
    }

    #[test]
    fn test_find() {
        let mut uf = UnionFind::new(10).unwrap();
        uf.unify(1, 2);
        assert_eq!(uf.find(1), uf.find(2));
    }

    #[test]
    fn test_connected() {
        let mut uf = UnionFind::new(10).unwrap();
        uf.unify(1, 2);
        assert!(uf.connected(1, 2));
        assert!(!uf.connected(1, 3));
    }

    #[test]
    fn test_component_size() {
        let mut uf = UnionFind::new(10).unwrap();
        uf.unify(1, 2);
        uf.unify(2, 3);
        assert_eq!(uf.component_size(1), 3);
        assert_eq!(uf.component_size(4), 1);
    }

    #[test]
    fn test_size() {
        let uf = UnionFind::new(10).unwrap();
        assert_eq!(uf.size(), 10);
    }

    #[test]
    fn test_components() {
        let mut uf = UnionFind::new(10).unwrap();
        assert_eq!(uf.components(), 10);
        uf.unify(1, 2);
        assert_eq!(uf.components(), 9);
        uf.unify(2, 3);
        assert_eq!(uf.components(), 8);
    }

    #[test]
    fn test_unify() {
        let mut uf = UnionFind::new(10).unwrap();
        uf.unify(1, 2);
        assert_eq!(uf.find(1), uf.find(2));
        uf.unify(2, 3);
        assert_eq!(uf.find(1), uf.find(3));
        assert_eq!(uf.components(), 8);
    }
}
