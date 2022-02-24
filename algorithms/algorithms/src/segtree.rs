use std::ops::Range;

#[derive(Debug, Clone)]
pub struct SegmentTree<T, F, U> {
    halflen: usize,
    a: Vec<T>,
    f: F,
    unit: U,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl<T, F, U> SegmentTree<T, F, U>
where
    T: Copy,
    F: Fn(T, T) -> T,
    U: Fn() -> T,
{
    pub fn new(len: usize, f: F, unit: U) -> Self {
        let halflen = len.next_power_of_two();
        Self {
            halflen,
            a: vec![unit(); halflen * 2],
            f,
            unit,
        }
    }
    pub fn from<I: ExactSizeIterator<Item = T>>(xs: I, f: F, unit: U) -> Self {
        let halflen = xs.len().next_power_of_two();
        let mut a = vec![unit(); halflen * 2];
        for (v, x) in a[halflen..].iter_mut().zip(xs) {
            *v = x;
        }
        let mut s = Self {
            halflen,
            a,
            f,
            unit,
        };
        s.update_all();
        s
    }
    pub fn set(&mut self, index: usize, value: T) {
        let mut i = self.halflen + index;
        self.a[i] = value;
        loop {
            i = i / 2;
            self.a[i] = (self.f)(self.a[i * 2], self.a[i * 2 + 1]);
            if i == 1 {
                break;
            }
        }
    }
    pub fn query(&self, r: Range<usize>) -> T {
        self._query(r.start, r.end, 1, 0, self.halflen)
    }
    fn _query(&self, start: usize, end: usize, i: usize, i_start: usize, i_end: usize) -> T {
        if end <= i_start || start >= i_end {
            (self.unit)()
        } else if start <= i_start && i_end <= end {
            self.a[i]
        } else {
            let mid = i_start + (i_end - i_start) / 2;
            (self.f)(
                self._query(start, end, i * 2, i_start, mid),
                self._query(start, end, i * 2 + 1, mid, i_end),
            )
        }
    }
    pub fn prefer_when(&self, side: Side, p: impl Fn(T) -> bool) -> Option<usize> {
        let mut i = 1;
        while i < self.halflen {
            let l = i * 2;
            let r = i * 2 + 1;
            let prefered = if side == Side::Left { l } else { r };
            let otherwise = l.wrapping_add(r).wrapping_sub(prefered);
            if p(self.a[prefered]) {
                i = prefered;
            } else if p(self.a[otherwise]) {
                i = otherwise;
            } else {
                return None;
            }
        }
        Some(i - self.halflen)
    }
    pub fn change_many(&mut self, func: impl FnOnce(&mut [T])) {
        func(&mut self.a[self.halflen..]);
        self.update_all();
    }
    fn update_all(&mut self) {
        for i in (1..self.halflen).rev() {
            self.a[i] = (self.f)(self.a[i * 2], self.a[i * 2 + 1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums() {
        let mut st = SegmentTree::new(8, |a, b| a + b, || 0);
        st.set(0, 7);
        st.set(1, 4);
        st.set(2, 5);
        st.set(3, 5);
        st.set(4, 0);
        st.set(5, 1);
        st.set(6, 4);
        st.set(7, 3);
        eprintln!("{:?}", st.a);
        assert_eq!(7 + 4 + 5 + 5 + 1 + 4 + 3, st.query(0..8));
        assert_eq!(7 + 4 + 5, st.query(0..3));
        assert_eq!(4 + 5 + 5, st.query(1..4));
        assert_eq!(5 + 1 + 4, st.query(3..7));
    }

    #[test]
    fn min() {
        let mut st = SegmentTree::new(5, |a: usize, b| a.min(b), || usize::MAX);
        let xs = [10, 11, 4, 8, 7];
        for (i, &x) in xs.iter().enumerate() {
            st.set(i, x);
        }
        eprintln!("{:?}", st.a);
        for i in 0..xs.len() {
            for j in i..xs.len() {
                assert_eq!(
                    *xs[i..j].iter().min().unwrap_or(&usize::MAX),
                    st.query(i..j),
                    "{}..{}",
                    i,
                    j,
                );
            }
        }
    }

    #[test]
    fn from() {
        let st = SegmentTree::from([4, 9, 1].into_iter(), |a, b| a + b, || 0);
        eprintln!("{:?}", st.a);
        assert_eq!(4, st.query(0..1));
        assert_eq!(9, st.query(1..2));
        assert_eq!(1, st.query(2..3));

        assert_eq!(13, st.query(0..2));
        assert_eq!(10, st.query(1..3));

        assert_eq!(14, st.query(0..3));
    }

    #[test]
    fn change_many() {
        let mut st = SegmentTree::from([1, 2, 3, 8].into_iter(), |a, b| a.max(b), || 0);
        eprintln!("{:?}", st.a);
        assert_eq!(8, st.query(0..4));
        st.change_many(|xs| {
            xs[0] = 5;
            xs[3] = 2;
        });
        eprintln!("{:?}", st.a);
        assert_eq!(5, st.query(0..4));
        assert_eq!(3, st.query(2..4));
    }

    #[test]
    fn prefer_when() {
        let st = SegmentTree::from([4, 3, 1, 1, 2].into_iter(), |a, b| a.min(b), || usize::MAX);
        eprintln!("{:?}", st.a);
        assert_eq!(Some(2), st.prefer_when(Side::Left, |x| x <= 1));
        assert_eq!(Some(2), st.prefer_when(Side::Left, |x| x <= 2));
        assert_eq!(Some(1), st.prefer_when(Side::Left, |x| x <= 3));
        assert_eq!(Some(0), st.prefer_when(Side::Left, |x| x <= 4));

        assert_eq!(Some(3), st.prefer_when(Side::Right, |x| x <= 1));
        assert_eq!(Some(4), st.prefer_when(Side::Right, |x| x <= 2));
    }

    extern crate test;
    use test::{black_box, Bencher};

    #[bench]
    fn query_req(b: &mut Bencher) {
        let l = 1024 * 1024;
        let mut st = SegmentTree::new(l, |a, b| a + b, || 0);
        for i in 0..l {
            st.set(i, black_box(69));
        }
        let es = [0.1, 0.2, 0.4, 0.6, 0.8, 1.0].map(|f| (l as f32 * f) as usize);
        b.iter(|| {
            for e in es {
                black_box(st.query(0..e));
            }
        });
    }
}
