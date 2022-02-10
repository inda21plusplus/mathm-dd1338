use std::ops::{AddAssign, Index, IndexMut, Sub};

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

impl<const N: usize, T: Copy + Default> FenwickTree<[T; N]> {
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
    T: Copy + AddAssign + Sub<Output = T> + Default,
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
}
