use crate::geometry::Scalar;
use std::{fmt, ops};

#[macro_export]
macro_rules! v {
    ($($x:expr),*) => {
        $crate::geometry::Vector::from([$($x, )*])
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector<T, const N: usize>([T; N])
where
    T: Scalar;

impl<T: Scalar, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;

    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Scalar, const N: usize> Vector<T, N> {
    pub fn len_sq(&self) -> T {
        self.into_iter().map(|x| x * x).sum()
    }
    // pub fn len(&self) -> T {
    //     self.len_sq().sqrt()
    // }
}

impl<T: Scalar> Vector<T, 2> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }
    pub fn x(&self) -> T {
        self[0]
    }
    pub fn y(&self) -> T {
        self[1]
    }
}

impl<T: Scalar> Vector<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
    pub fn x(&self) -> T {
        self[0]
    }
    pub fn y(&self) -> T {
        self[1]
    }
    pub fn z(&self) -> T {
        self[2]
    }
}

impl<T: Scalar> Vector<T, 4> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self([x, y, z, w])
    }
    pub fn x(&self) -> T {
        self[0]
    }
}

impl<T: Scalar, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(vs: [T; N]) -> Self {
        Self(vs)
    }
}

impl<T: Scalar> Vector<T, 2> {
    pub fn cross(&self, rhs: Self) -> T {
        self[0] * rhs[1] - self[1] * rhs[0]
    }
}

impl<T: Scalar> Vector<T, 3> {
    pub fn cross(&self, rhs: &Self) -> Self {
        Self([
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ])
    }
}

impl<T: Scalar, const N: usize> ops::Add for Vector<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.zip(rhs.0).map(|(a, b)| a + b))
    }
}

impl<T: Scalar, const N: usize> ops::Sub for Vector<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.zip(rhs.0).map(|(a, b)| a - b))
    }
}

impl<T: Scalar, const N: usize> ops::Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Scalar, const N: usize> ops::IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Scalar, const N: usize> fmt::Debug for Vector<T, N>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..N {
            write!(f, "{}{:?}", if i > 0 { " " } else { "" }, self[i])?;
        }
        write!(f, "]")
    }
}

impl<T: Scalar, const N: usize> fmt::Display for Vector<T, N>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..N {
            write!(f, "{}{}", if i > 0 { " " } else { "" }, self[i])?;
        }
        write!(f, "]")
    }
}
