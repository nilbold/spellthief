use crate::math::Vector;

/// The `Spatial` component defines a location in space.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Spatial(pub Vector);
