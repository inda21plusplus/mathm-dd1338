// Heres an implementation in APL:
// ⎕IO ← 0
// bits       ← 2∘⊥⍣¯1
// lsb        ← 2⊥⊢(^⍥bits)-

// dnidxs     ← {⍵<0 :⍬⋄ (0≥⊃⌽⍵) :    ⍵ ⋄  ∇⍵,(⊢-lsb)¯1∘↑⍵}
// upidxs     ← {⍵≡0 :0 ⋄ (⍺≤⊃⌽⍵) : ¯1↓⍵ ⋄ ⍺∇⍵,(⊢+lsb)¯1∘↑⍵}
// inc        ← {⍵+(⍳∊upidxs∘⍺)≢⍵} ⍝ O(n) time complexity (which destroys the whole purpose)
// sum        ← {+/⍵[dnidxs ⍺-1]}  ⍝ O(log n) time complexity

use std::ops::{AddAssign, Index, IndexMut};

fn lsb(x: usize) -> usize {
    (x as isize & -(x as isize)) as usize
}

pub struct FenwickTree<A = Vec<usize>> {
    pub a: A,
}

impl<T: Copy + Default> FenwickTree<Vec<T>> {
    pub fn new(size: usize) -> Self {
        Self {
            a: vec![T::default(); size],
        }
    }
}

impl<T: Copy + Default, const N: usize> FenwickTree<[T; N]> {
    pub fn new() -> Self {
        Self {
            a: [T::default(); N],
        }
    }
}

impl<A, T> FenwickTree<A>
where
    A: Index<usize, Output = T>,
    A: IndexMut<usize>,
    A: AsRef<[T]>, // for .len()
    T: Copy + AddAssign + Default,
{
    pub fn prefix_sum(&self, mut i: usize) -> T {
        i = match i.checked_sub(1) {
            Some(i) => i,
            None => return T::default(), // there is no zero trait in std
        };
        let mut sum = self.a[0];
        while i != 0 {
            sum += self.a[i];
            i -= lsb(i);
        }
        sum
    }

    pub fn add(&mut self, mut i: usize, d: T) {
        if i == 0 {
            self.a[0] += d;
            return;
        }
        while i < self.a.as_ref().len() {
            self.a[i] += d;
            i += lsb(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mut tree = FenwickTree::<Vec<usize>>::new(10);

        tree.add(7, 23);
        assert_eq!(23, tree.prefix_sum(8));
        tree.add(3, 17);
        assert_eq!(40, tree.prefix_sum(8));
    }

    #[test]
    fn sample2() {
        let mut tree = FenwickTree::<[isize; 5]>::new();

        tree.add(0, -43);
        tree.add(4, 1);
        assert_eq!(0, tree.prefix_sum(0));
        assert_eq!(-42, tree.prefix_sum(5));
    }

    #[test]
    fn testetest() {
        let mut tree = FenwickTree::<Vec<usize>>::new(33);
        tree.add(1, 1);
        let idxs: Vec<usize> = tree
            .a
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| (v > 0).then(|| i))
            .collect();
        assert_eq!(&[1, 2, 4, 8, 16, 32], idxs.as_slice());
    }
}
