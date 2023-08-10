use crate::math::Vector;

/// The `Spatial` component defines a location in space.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Spatial(pub Vector);

impl Spatial {
    /// Provides screen (pixel) coordinates.
    pub fn to_screen(&self) -> (i32, i32) {
        let pos = &self.0;
        (pos.0 / 16, pos.1 / 16)
    }
}
