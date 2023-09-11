use hecs::{Query, Without};
use log::debug;

use crate::component::{Controller, Player};
use crate::game::event::physics::StaticCollision;
use crate::game::event::EventIter;
use crate::game::{Events, State};
use crate::math::Scaled;

#[derive(Query)]
struct NpcQuery<'a> {
    conn: &'a mut Controller,
}

impl State {
    pub fn process_npcs(&mut self, _events: &mut Events) {
        for (_id, npc) in self.world.query_mut::<Without<NpcQuery, &Player>>() {
            npc.conn.moving = npc.conn.on_floor;
        }
    }

    pub fn event_npc_static_collision(&mut self, events: EventIter<'_, StaticCollision>) {
        for event in events {
            if event.entity == self.player {
                continue;
            }

            if event.normal.x.abs() > Scaled::zero() {
                debug!("npc ({}) bonk", event.entity.id());
                if let Ok(conn) = self.world.query_one_mut::<&mut Controller>(event.entity) {
                    conn.direction = conn.direction.flip();
                }
            }
        }
    }
}
