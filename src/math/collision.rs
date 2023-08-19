use crate::math::Vector;

pub use aabb::BoundingBox;

pub mod aabb;

/// Defines the resulting 'Hit' for a collision check.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Hit {
    /// Point of contact.
    pub pos: Vector,
    /// Overlap between two objets.
    pub delta: Vector,
    /// Surface normal at the point of contact.
    pub normal: Vector,
}
