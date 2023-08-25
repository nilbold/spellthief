use crate::component::{Collision, Controller, Physics, Spatial};
use crate::game::State;
use crate::math::{collision::BoundingBox, Scaled, Vector};

impl State {
    pub fn physics_step(&mut self) {
        // hardcoding screen half width/height and wall thickness here for testing
        const HWIDTH: i32 = 160;
        const HHEIGHT: i32 = 120;
        const THICK: i32 = 20;

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
                phys.vel.y = Scaled::from(3);
                phys.on_floor = false;
                conn.pre_jump = 0;
            }

            // hardcoding timesteps here for now
            const DELTA: i32 = 1000 / 20;
            const GRAVITY: Scaled = Scaled::new(98, 1);

            let mut acc = Vector::from((Scaled::zero(), -GRAVITY / DELTA));

            // lets apply friction
            let friction = {
                if phys.on_floor {
                    GRAVITY / DELTA
                } else {
                    GRAVITY / DELTA / 2
                }
            };
            if !phys.vel.x.is_zero() {
                let vel_p = phys.vel.x.is_positive();
                let s = if vel_p { -1 } else { 1 };
                acc.x = acc.x + friction * s;
                if (vel_p && (phys.vel.x + acc.x).is_negative())
                    || (!vel_p && (phys.vel.x + acc.x).is_positive())
                {
                    acc.x = -phys.vel.x;
                }
            }

            phys.vel = phys.vel + acc;

            const LIMIT: Scaled = Scaled::new(200, 0);

            phys.vel.x = phys.vel.x.min(LIMIT).max(-LIMIT);
            phys.vel.y = phys.vel.y.min(LIMIT).max(-LIMIT);

            *pos = *pos + phys.vel;

            let speed = phys.speed / DELTA;
            if conn.moving {
                use crate::component::controller::MoveDirection::*;
                match conn.direction {
                    Left => *pos = *pos + Vector::from((-speed, 0)),
                    Right => *pos = *pos + Vector::from((speed, 0)),
                }
            }

            for wall in walls.iter() {
                let bounds = coll.bounds + pos.0;
                if let Some(hit) = bounds.overlap(wall) {
                    *pos = *pos - hit.delta;
                    if hit.normal.y == -Scaled::from(1) {
                        phys.on_floor = true;
                        phys.vel.y = Scaled::zero();
                    }
                }
            }

            conn.reset();
        }
    }
}
