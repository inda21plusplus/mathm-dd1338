use std::fmt;

use super::{Scalar, Vector};

pub struct AnyLine<T: Scalar, const N: usize, const INFINITE: bool>(
    pub Vector<T, N>,
    pub Vector<T, N>,
);

pub type Line<T, const N: usize> = AnyLine<T, N, true>;
pub type LineSegment<T, const N: usize> = AnyLine<T, N, false>;

impl<T: Scalar, const INFINITE: bool> AnyLine<T, 2, INFINITE> {
    pub fn new(a: Vector<T, 2>, b: Vector<T, 2>) -> Self {
        Self(a, b)
    }
    pub fn side(&self, v: Vector<T, 2>) -> Side {
        match (self.1 - self.0).cross(v - self.0) {
            c if c > T::ZERO => Side::Left,
            c if c < T::ZERO => Side::Right,
            _ => Side::On,
        }
    }
}

impl<T: Scalar, const N: usize> LineSegment<T, N> {
    pub fn len_sq(&self) -> T {
        (self.0 - self.1).into_iter().map(|x| x * x).sum()
    }
}

impl<T: Scalar, const N: usize> fmt::Debug for LineSegment<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(| {:?} --> {:?} |)", self.0, self.1)
    }
}

impl<T: Scalar, const N: usize> fmt::Debug for Line<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(--> {:?} --> {:?} -->)", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Left,
    On,
    Right,
}
