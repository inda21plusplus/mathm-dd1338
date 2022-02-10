use std::ops::{Index, IndexMut};

pub struct UnionFind<A = Vec<usize>> {
    parent: A,
    rank: A,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }
}

impl<const N: usize> UnionFind<[usize; N]> {
    pub fn new() -> Self {
        let mut parent = [0; N];
        for (i, p) in parent.iter_mut().enumerate() {
            *p = i;
        }
        Self {
            parent,
            rank: [0; N],
        }
    }
}

impl<A> UnionFind<A>
where
    A: Index<usize, Output = usize>,
    A: IndexMut<usize>,
{
    pub fn root(&mut self, mut node: usize) -> usize {
        let mut r = node;
        while r != self.parent[r] {
            r = self.parent[r];
        }
        while node != r {
            let tmp = node;
            node = self.parent[node];
            self.parent[tmp] = r;
        }
        r
    }

    pub fn join(&mut self, a: usize, b: usize) {
        let mut a = self.root(a);
        let mut b = self.root(b);

        if a == b {
            return;
        }

        if self.rank[a] < self.rank[b] {
            let tmp = a;
            a = b;
            b = tmp;
        }

        self.parent[b] = a;
        if self.rank[a] == self.rank[b] {
            self.rank[a] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_disjoint() {
        let mut uf = UnionFind::<Vec<usize>>::new(5);
        for a in 0..5 {
            for b in 0..5 {
                assert!((a == b) == (uf.root(a) == uf.root(b)));
            }
        }
    }

    #[test]
    fn join() {
        let mut uf = UnionFind::<[usize; 4]>::new();
        assert_ne!(uf.root(0), uf.root(1));
        uf.join(0, 1);
        assert_eq!(uf.root(0), uf.root(1));
        assert_ne!(uf.root(0), uf.root(2));
        uf.join(1, 2);
        assert_eq!(uf.root(1), uf.root(2));
        assert_eq!(uf.root(0), uf.root(2));
    }
}
