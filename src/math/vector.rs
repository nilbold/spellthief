use std::borrow::Borrow;
use std::ops::{Add, Sub};

/// Defines a magnitude and direction in 2d space.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    /// How many fractions per unit is used with fixed point scaling.
    ///
    /// This is defined in powers of two, ie. F = 4, 2^F = 16.
    pub const F: i32 = 5;

    pub fn new(x: i32, y: i32) -> Self {
        Vector { x, y }
    }

    pub fn zero() -> Self {
        Vector::new(0, 0)
    }

    /// The vector is screen space.
    pub fn screen(&self) -> Self {
        Vector::new(self.x >> Vector::F, self.y >> Vector::F)
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
        let a = Vector::new(1, 2);
        let b = Vector::new(2, 1);
        let c = Vector::new(1, -1);

        assert_eq!(a - b + c, Vector::zero());
        assert_eq!(&a - &b + &c, Vector::zero());
    }
}
