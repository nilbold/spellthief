use hecs::Entity;

use crate::math::Vector;

use super::{Event, EventQueue};

#[derive(Copy, Clone)]
pub struct StaticCollision {
    pub entity: Entity,
    pub normal: Vector,
}

impl Event for StaticCollision where Self: Send + Sync + 'static {}

impl EventQueue<StaticCollision> {
    pub fn static_collision(&mut self, entity: Entity, normal: Vector) {
        self.send(StaticCollision { entity, normal })
    }
}
