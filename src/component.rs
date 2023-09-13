pub use animation::Animation;
pub use collision::Collision;
pub use controller::Controller;
pub use coyote::CoyoteTime;
pub use physics::Physics;
pub use spatial::Spatial;
pub use sprite::Sprite;

pub mod animation;
pub mod collision;
pub mod controller;
pub mod coyote;
pub mod physics;
pub mod spatial;
pub mod sprite;

/// The `Player` component marks an entity as the player.
pub struct Player;
