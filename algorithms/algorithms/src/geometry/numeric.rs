use std::{fmt, ops};

pub trait Numeric:
    Sized
    + Copy
    + fmt::Debug
    + fmt::Display
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + PartialOrd
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_numeric {
    ($t:ty, $z:literal, $o:literal) => {
        impl Numeric for $t {
            const ZERO: $t = $z;
            const ONE: $t = $o;
        }
    };
}

impl_numeric!(u8, 0, 1);
impl_numeric!(i8, 0, 1);
impl_numeric!(u16, 0, 1);
impl_numeric!(i16, 0, 1);
impl_numeric!(u32, 0, 1);
impl_numeric!(i32, 0, 1);
impl_numeric!(u64, 0, 1);
impl_numeric!(i64, 0, 1);
impl_numeric!(u128, 0, 1);
impl_numeric!(i128, 0, 1);
impl_numeric!(f32, 0., 1.);
impl_numeric!(f64, 0., 1.);
impl_numeric!(isize, 0, 1);
impl_numeric!(usize, 0, 1);
