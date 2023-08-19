use crate::math::{collision::BoundingBox, Vector};

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Collision {
    pub bounds: BoundingBox,
}

impl Collision {
    pub fn new<V: Into<Vector>>(pos: V, dim: V) -> Self {
        Collision {
            bounds: BoundingBox::new(pos, dim),
        }
    }
}
