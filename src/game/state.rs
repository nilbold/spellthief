use hecs::{Entity, World};
use log::debug;

use crate::component::{Collision, Physics, Player, Spatial};
use crate::math::Vector;

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
            Physics::new(10, 10),
            Collision::new(-10 << Vector::F, 0, 10 << Vector::F, 28 << Vector::F),
        ));
        debug!("player entity generated ({})", player.id());

        State { world, player }
    }

    pub fn physics_step(&mut self) {
        let wall_right = 160 << Vector::F;
        let wall_left = -wall_right;
        let wall_top = 120 << Vector::F;
        let wall_bottom = -wall_top;

        for (_id, (pos, phys, coll)) in self
            .world
            .query_mut::<(&mut Spatial, &mut Physics, &Collision)>()
        {
            *pos = *pos + phys.vel;

            if pos.y + coll.bounds.max.y > wall_top {
                pos.y = wall_top - coll.bounds.max.y;
                phys.vel.y = -phys.vel.y;
            }
            if pos.y + coll.bounds.min.y < wall_bottom {
                pos.y = wall_bottom - coll.bounds.min.y;
                phys.vel.y = -phys.vel.y;
            }
            if pos.x + coll.bounds.max.x > wall_right {
                pos.x = wall_right - coll.bounds.max.x;
                phys.vel.x = -phys.vel.x;
            }
            if pos.x + coll.bounds.min.x < wall_left {
                pos.x = wall_left - coll.bounds.min.x;
                phys.vel.x = -phys.vel.x;
            }
        }
    }
}
