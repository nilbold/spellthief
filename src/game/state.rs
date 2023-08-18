use hecs::{Entity, World};
use log::debug;

use crate::component::{Collision, Physics, Player, Spatial};

/// Game state.
pub struct State {
    pub world: World,
    pub player: Entity,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();
        let player = world.spawn((
            Player,
            Spatial::new(0, 0),
            Physics::new(4, 4),
            Collision::new(0, 0, 20, 28),
        ));
        debug!("player entity generated ({})", player.id());

        State { world, player }
    }

    pub fn physics_step(&mut self) {
        for (_id, (pos, phys)) in self.world.query_mut::<(&mut Spatial, &Physics)>() {
            *pos = *pos + phys.vel;
        }

        for (_id, (_pos, _coll)) in self.world.query_mut::<(&mut Spatial, &Collision)>() {}
    }
}
