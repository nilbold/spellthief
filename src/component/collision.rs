use crate::math::{BoundingBox, Vector};

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Collision {
    pub bounds: BoundingBox,
}

impl Collision {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Collision {
            bounds: BoundingBox::new(Vector::new(min_x, min_y), Vector::new(max_x, max_y)),
        }
    }
}
