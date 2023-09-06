use crate::math::Vector;

/// The `Physics` component allows forces to be applied.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Physics {
    pub vel: Vector,
    pub speed: i32,
}

impl Physics {
    pub fn new(x: i32, y: i32) -> Self {
        Physics {
            vel: Vector::from((x, y)),
            speed: 100,
        }
    }
}
