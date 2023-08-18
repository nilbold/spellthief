use std::borrow::Borrow;
use std::ops::{Add, Deref, DerefMut, Sub};

use crate::math::Vector;

/// The `Spatial` component defines a location in space.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Spatial(pub Vector);

impl Spatial {
    pub fn new(x: i32, y: i32) -> Self {
        Spatial(Vector::new(x, y))
    }
}

impl Deref for Spatial {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Spatial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, B> Add<B> for &'a Spatial
where
    B: Borrow<Vector>,
{
    type Output = Spatial;

    fn add(self, other: B) -> Self::Output {
        Spatial(self.0 + other.borrow())
    }
}

impl<B> Add<B> for Spatial
where
    B: Borrow<Vector>,
{
    type Output = Spatial;

    #[allow(clippy::op_ref)]
    fn add(self, other: B) -> Self::Output {
        &self + other
    }
}

impl<'a, B> Sub<B> for &'a Spatial
where
    B: Borrow<Vector>,
{
    type Output = Spatial;

    fn sub(self, other: B) -> Self::Output {
        Spatial(self.0 - other.borrow())
    }
}

impl<B> Sub<B> for Spatial
where
    B: Borrow<Vector>,
{
    type Output = Spatial;

    #[allow(clippy::op_ref)]
    fn sub(self, other: B) -> Self::Output {
        &self - other
    }
}
