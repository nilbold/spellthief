use std::borrow::Borrow;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Fixed point scaled value.
///
/// Scaled by a pre-determined fractions per unit, `Scaled::F`.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Scaled(i32);

impl Scaled {
    /// How many fractions per unit is used with fixed point scaling.
    ///
    /// This is defined in powers of two, ie. F = 4, 2^F = 16.
    pub const F: u8 = 16;

    pub const fn new(value: i32, precision: u32) -> Self {
        Scaled((value << Scaled::F) / 10_i32.pow(precision))
    }

    pub const fn abs(self) -> Self {
        Scaled(self.0.abs())
    }

    pub const fn zero() -> Self {
        Scaled(0)
    }

    pub const fn unit(self) -> i32 {
        self.0 >> Scaled::F
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }
}

macro_rules! from_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for Scaled {
            fn from(item: $t) -> Self {
                Scaled((item as i32) << Scaled::F)
            }
        }
    )*)
}

from_impl! { u8 u16 u32 i8 i16 i32 }

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl Mul::<$t> for Scaled
        {
            type Output = Scaled;

            fn mul(self, other: $t) -> Self::Output {
                Scaled(self.0 * other as i32)
            }
        }
    )*)
}

mul_impl! { u8 u16 u32 i8 i16 i32 }

macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl Div::<$t> for Scaled
        {
            type Output = Scaled;

            fn div(self, other: $t) -> Self::Output {
                Scaled(self.0 / other as i32)
            }
        }
    )*)
}

div_impl! { u8 u16 u32 i8 i16 i32 }

impl<'a, B> Add<B> for &'a Scaled
where
    B: Borrow<Scaled>,
{
    type Output = Scaled;

    fn add(self, other: B) -> Self::Output {
        Scaled(self.0 + other.borrow().0)
    }
}

impl<B> Add<B> for Scaled
where
    B: Borrow<Scaled>,
{
    type Output = Scaled;

    #[allow(clippy::op_ref)]
    fn add(self, other: B) -> Self::Output {
        &self + other
    }
}

impl<'a, B> Sub<B> for &'a Scaled
where
    B: Borrow<Scaled>,
{
    type Output = Scaled;

    fn sub(self, other: B) -> Self::Output {
        Scaled(self.0 - other.borrow().0)
    }
}

impl<B> Sub<B> for Scaled
where
    B: Borrow<Scaled>,
{
    type Output = Scaled;

    #[allow(clippy::op_ref)]
    fn sub(self, other: B) -> Self::Output {
        &self - other
    }
}

impl Neg for Scaled {
    type Output = Scaled;

    fn neg(self) -> Scaled {
        Scaled(-self.0)
    }
}
