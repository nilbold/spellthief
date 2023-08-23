use crate::component::{Collision, Controller, Physics, Spatial};
use crate::game::State;
use crate::math::{collision::BoundingBox, Vector};

impl State {
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

        for (_id, (pos, phys, conn, coll)) in
            self.world
                .query_mut::<(&mut Spatial, &mut Physics, &mut Controller, &Collision)>()
        {
            if phys.on_floor && (conn.jumping || conn.pre_jump > 0) {
                phys.vel.y = 3 * (1 << Vector::F);
                phys.on_floor = false;
                conn.pre_jump = 0;
            }

            // hardcoding timesteps here for now
            const DELTA: i32 = 1000 / 20;
            const GRAVITY: i32 = 98 * (1 << Vector::F) / 10;

            let speed = phys.speed / DELTA;
            let mut acc = Vector::new(0, -GRAVITY / DELTA);

            // lets apply friction
            const GROUND: i32 = GRAVITY / DELTA;
            const AIR: i32 = GRAVITY / DELTA / 2;
            if phys.vel.x != 0 {
                if phys.vel.x.is_positive() {
                    acc.x -= if phys.on_floor { GROUND } else { AIR };
                    if phys.vel.x + acc.x < 0 {
                        acc.x = -phys.vel.x;
                    }
                } else {
                    acc.x += if phys.on_floor { GROUND } else { AIR };
                    if phys.vel.x + acc.x > 0 {
                        acc.x = -phys.vel.x;
                    }
                }
            }

            phys.vel = phys.vel + acc;

            const LIMIT: i32 = 200 * (1 << Vector::F);

            phys.vel.x = phys.vel.x.min(LIMIT).max(-LIMIT);
            phys.vel.y = phys.vel.y.min(LIMIT).max(-LIMIT);

            *pos = *pos + phys.vel;

            if conn.moving {
                use crate::component::controller::MoveDirection::*;
                match conn.direction {
                    Left => *pos = *pos + Vector::new(-speed, 0),
                    Right => *pos = *pos + Vector::new(speed, 0),
                }
            }

            for wall in walls.iter() {
                let bounds = coll.bounds + pos.0;
                if let Some(hit) = bounds.overlap(wall) {
                    *pos = *pos - hit.delta;
                    if hit.normal.y == -(1 << Vector::F) {
                        phys.on_floor = true;
                        phys.vel.y = 0;
                    }
                }
            }

            conn.reset();
        }
    }
}
