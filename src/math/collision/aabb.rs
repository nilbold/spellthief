use std::borrow::Borrow;
use std::ops::{Add, Sub};

use crate::math::collision::Hit;
use crate::math::Vector;

/// Axis-Aligned Bounding Box
///
/// Defined by a single point, and its dimensions (half size for each axis)
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct BoundingBox {
    pub pos: Vector,
    pub dim: Vector,
}

impl BoundingBox {
    pub fn new<V: Into<Vector>>(pos: V, dim: V) -> Self {
        BoundingBox {
            pos: pos.into(),
            dim: dim.into(),
        }
    }

    pub fn overlap(&self, other: &BoundingBox) -> Option<Hit> {
        let dx = other.pos.x - self.pos.x;
        let px = (other.dim.x + self.dim.x) - dx.abs();

        if px <= 0 {
            return None;
        }

        let dy = other.pos.y - self.pos.y;
        let py = (other.dim.y + self.dim.y) - dy.abs();
        if py <= 0 {
            return None;
        }

        let mut hit = Hit::default();

        if px < py {
            let sx = if dx.is_positive() { 1 } else { -1 };
            hit.delta.x = px * sx;
            hit.normal.x = sx << Vector::F;
            hit.pos.x = self.pos.x + self.dim.x * sx;
            hit.pos.y = other.pos.y;
        } else {
            let sy = if dy.is_positive() { 1 } else { -1 };
            hit.delta.y = py * sy;
            hit.normal.y = sy;
            hit.pos.x = other.pos.x;
            hit.pos.y = self.pos.y + self.dim.y * sy;
        }

        Some(hit)
    }
}

impl<'a, B> Add<B> for &'a BoundingBox
where
    B: Borrow<Vector>,
{
    type Output = BoundingBox;

    fn add(self, other: B) -> Self::Output {
        BoundingBox {
            pos: self.pos + other.borrow(),
            dim: self.dim,
        }
    }
}

impl<B> Add<B> for BoundingBox
where
    B: Borrow<Vector>,
{
    type Output = BoundingBox;

    #[allow(clippy::op_ref)]
    fn add(self, other: B) -> Self::Output {
        &self + other
    }
}

impl<'a, B> Sub<B> for &'a BoundingBox
where
    B: Borrow<Vector>,
{
    type Output = BoundingBox;

    fn sub(self, other: B) -> Self::Output {
        BoundingBox {
            pos: self.pos - other.borrow(),
            dim: self.dim,
        }
    }
}

impl<B> Sub<B> for BoundingBox
where
    B: Borrow<Vector>,
{
    type Output = BoundingBox;

    #[allow(clippy::op_ref)]
    fn sub(self, other: B) -> Self::Output {
        &self - other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounding_box_overlap() {
        let a = BoundingBox::new((5, 5), (5, 5));
        let b = BoundingBox::new((-15, -15), (5, 5));
        let c = BoundingBox::new((-5, -5), (10, 10));

        assert_eq!(a.overlap(&b), None);
        assert_eq!(b.overlap(&a), None);
        assert!(a.overlap(&c).is_some());
        assert!(b.overlap(&c).is_some());
    }

    #[test]
    fn bounding_box_offset() {
        let a = BoundingBox::new((-5, 0), (5, 5));
        let b = BoundingBox::new((5, 0), (5, 5));

        assert_eq!(a.overlap(&b), None);

        let offset = Vector::new(8, 0);

        assert!((a + offset).overlap(&b).is_some());
    }
}
