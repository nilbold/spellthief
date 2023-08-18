pub use collision::Collision;
pub use physics::Physics;
pub use spatial::Spatial;

pub mod collision;
pub mod physics;
pub mod spatial;

/// The `Player` component marks an entity as the player.
pub struct Player;
