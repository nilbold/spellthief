use hecs::{Entity, World};
use log::debug;

use crate::component::{Collision, Physics, Player, Spatial};
use crate::math::{collision::BoundingBox, Vector};

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
            Physics::new(100, 100),
            Collision::new((0, 0), (10 << Vector::F, 14 << Vector::F)),
        ));
        debug!("player entity generated ({})", player.id());

        State { world, player }
    }

    pub fn physics_step(&mut self) {
        // hardcoding screen half width/height and wall thickness here for testing
        const HWIDTH: i32 = 160 << Vector::F;
        const HHEIGHT: i32 = 120 << Vector::F;
        const THICK: i32 = 20 << Vector::F;

        let walls = [
            BoundingBox::new((HWIDTH + THICK, 0), (THICK, HHEIGHT)),
            BoundingBox::new((-(HWIDTH + THICK), 0), (THICK, HHEIGHT)),
            BoundingBox::new((0, HHEIGHT + THICK), (HWIDTH, THICK)),
            BoundingBox::new((0, -(HHEIGHT + THICK)), (HWIDTH, THICK)),
        ];

        for (_id, (pos, phys, coll)) in self
            .world
            .query_mut::<(&mut Spatial, &mut Physics, &Collision)>()
        {
            *pos = *pos + phys.vel;

            for wall in walls.iter() {
                let bounds = coll.bounds + pos.0;
                if let Some(hit) = bounds.overlap(wall) {
                    *pos = *pos - hit.delta;
                    if hit.delta.x.abs() > 0 {
                        phys.vel.x = -phys.vel.x;
                    } else if hit.delta.y.abs() > 0 {
                        phys.vel.y = -phys.vel.y;
                    }
                }
            }
        }
    }
}
