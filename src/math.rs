#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Vector(pub i32, pub i32);

impl Vector {
    pub const ZERO: Vector = Vector(0, 0);
}
