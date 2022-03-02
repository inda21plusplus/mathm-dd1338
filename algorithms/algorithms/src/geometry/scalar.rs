use std::iter::Sum;
use std::{fmt, ops};

pub trait Scalar:
    Sized
    + Copy
    + fmt::Debug
    + fmt::Display
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + PartialOrd
    + PartialEq
    + Sum
{
    const ZERO: Self;
    const ONE: Self;

    fn abs(self) -> Self;
}

macro_rules! impl_numeric {
    ($t:ty, $z:literal, $o:literal) => {
        impl Scalar for $t {
            const ZERO: $t = $z;
            const ONE: $t = $o;

            fn abs(self) -> Self {
                self.abs()
            }
        }
    };
}

impl_numeric!(i8, 0, 1);
impl_numeric!(i16, 0, 1);
impl_numeric!(i32, 0, 1);
impl_numeric!(i64, 0, 1);
impl_numeric!(i128, 0, 1);
// impl_numeric!(f32, 0., 1.);
// impl_numeric!(f64, 0., 1.);
impl_numeric!(isize, 0, 1);
