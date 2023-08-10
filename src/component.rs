use crate::math::Vector;

/// the `Spatial` component defines a location in space.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Spatial(pub Vector);

impl Spatial {
    pub fn to_screen(&self) -> (i32, i32) {
        let pos = &self.0;
        (pos.0 / 16, pos.1 / 16)
    }
}
