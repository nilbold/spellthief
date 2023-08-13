use hecs::{Entity, World};
use log::debug;

use crate::component::{Physics, Player, Spatial};

/// Game state.
pub struct State {
    pub world: World,
    pub player: Entity,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();
        let player = world.spawn((Player, Spatial::new(0, 0), Physics::default()));
        debug!("player entity generated ({})", player.id());

        State { world, player }
    }
}
