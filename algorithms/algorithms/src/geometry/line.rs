use std::fmt;

use super::{Numeric, Vector};

pub struct Line<T: Numeric, const N: usize, const INFINITE: bool = true>(
    pub Vector<T, N>,
    pub Vector<T, N>,
);

pub type LineSegment<T, const N: usize> = Line<T, N, false>;

impl<T: Numeric, const INFINITE: bool> Line<T, 2, INFINITE> {
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

impl<T: Numeric, const N: usize> fmt::Debug for Line<T, N, false> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(| {:?} --> {:?} |)", self.0, self.1)
    }
}

impl<T: Numeric, const N: usize> fmt::Debug for Line<T, N, true> {
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
