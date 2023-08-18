use crate::math::Vector;

/// Axis-Aligned Bounding Box
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct BoundingBox {
    pub min: Vector,
    pub max: Vector,
}

impl BoundingBox {
    pub fn new(min: Vector, max: Vector) -> Self {
        BoundingBox { min, max }
    }
}
