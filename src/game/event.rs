use physics::StaticCollision;

pub mod physics;

/// Defines a type that can be used with the events system
pub trait Event: Send + Sync + 'static {}

pub type EventIter<'a, E> = std::slice::Iter<'a, E>;

/// Queue of events.
///
/// This is double buffered to allow events to be read from the prior frame.
#[derive(Clone)]
pub struct EventQueue<E: Event> {
    inner: (Vec<E>, Vec<E>),
}

impl<E: Event> EventQueue<E> {
    pub fn new() -> Self {
        EventQueue {
            inner: Default::default(),
        }
    }

    /// Sends an event to the current event queue buffer.
    pub fn send(&mut self, value: E) {
        self.inner.0.push(value);
    }

    pub fn read(&self) -> EventIter<'_, E> {
        self.inner.1.iter()
    }

    /// Swaps the event buffers.
    pub fn update(&mut self) {
        std::mem::swap(&mut self.inner.0, &mut self.inner.1);
        self.inner.0.clear();
    }

    /// Clears all events.
    pub fn clear(&mut self) {
        self.inner.0.clear();
        self.inner.1.clear();
    }

    #[inline]
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.inner.0.len() + self.inner.1.len()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Collection of event queues for all possible event types.
#[derive(Clone)]
pub struct Events {
    pub physics: EventQueue<StaticCollision>,
}

impl Events {
    pub fn new() -> Self {
        Events {
            physics: EventQueue::new(),
        }
    }

    pub fn update(&mut self) {
        self.physics.update();
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.physics.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector;

    use super::*;

    use hecs::Entity;

    #[test]
    fn queue() {
        let mut physics_events: EventQueue<StaticCollision> = EventQueue::new();

        physics_events.update();

        physics_events.static_collision(Entity::DANGLING, Vector::zero());
        physics_events.static_collision(Entity::DANGLING, Vector::zero());

        assert_eq!(physics_events.len(), 2);

        physics_events.update();

        physics_events.static_collision(Entity::DANGLING, Vector::zero());

        assert_eq!(physics_events.len(), 3);

        physics_events.clear();

        assert_eq!(physics_events.len(), 0);
    }
}
