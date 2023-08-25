use std::borrow::Borrow;
use std::convert::From;
use std::ops::{Add, Sub};

use crate::math::Scaled;

/// Defines a magnitude and direction in 2d space.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Vector {
    pub x: Scaled,
    pub y: Scaled,
}

impl Vector {
    pub const fn zero() -> Self {
        Vector {
            x: Scaled::zero(),
            y: Scaled::zero(),
        }
    }

    /// The vector in screen space.
    pub const fn screen(&self) -> (i32, i32) {
        (self.x.unit(), self.y.unit())
    }
}

macro_rules! from_impl {
    ($($t:ty)*) => ($(
        impl From<($t, $t)> for Vector {
            fn from(item: ($t, $t)) -> Self {
                Vector {
                    x: Scaled::from(item.0),
                    y: Scaled::from(item.1),
                }
            }
        }
    )*)
}

from_impl! { u8 u16 u32 i8 i16 i32 }

impl From<(Scaled, Scaled)> for Vector {
    fn from(item: (Scaled, Scaled)) -> Self {
        Vector {
            x: item.0,
            y: item.1,
        }
    }
}

impl<'a, B> Add<B> for &'a Vector
where
    B: Borrow<Vector>,
{
    type Output = Vector;

    fn add(self, other: B) -> Self::Output {
        Vector {
            x: self.x + other.borrow().x,
            y: self.y + other.borrow().y,
        }
    }
}

impl<B> Add<B> for Vector
where
    B: Borrow<Vector>,
{
    type Output = Vector;

    #[allow(clippy::op_ref)]
    fn add(self, other: B) -> Self::Output {
        &self + other
    }
}

impl<'a, B> Sub<B> for &'a Vector
where
    B: Borrow<Vector>,
{
    type Output = Vector;

    fn sub(self, other: B) -> Self::Output {
        Vector {
            x: self.x - other.borrow().x,
            y: self.y - other.borrow().y,
        }
    }
}

impl<B> Sub<B> for Vector
where
    B: Borrow<Vector>,
{
    type Output = Vector;

    #[allow(clippy::op_ref)]
    fn sub(self, other: B) -> Self::Output {
        &self - other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_ops() {
        let a = Vector::from((1, 2));
        let b = Vector::from((2, 1));
        let c = Vector::from((1, -1));

        assert_eq!(a - b + c, Vector::zero());
        assert_eq!(&a - &b + &c, Vector::zero());
    }
}
