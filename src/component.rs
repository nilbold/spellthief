use std::ops::Deref;

use crate::math::Vector;

/// The `Player` component marks an entity as the player.
pub struct Player;

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

/// The `Physics` component allows forces to be applied.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Physics {
    pub vel: Vector,
}
