use std::time::Duration;

use hecs::{Entity, World};
use log::debug;

use winit::event::VirtualKeyCode;

use crate::component::{
    Animation, Collision, Controller, CoyoteTime, Physics, Player, Spatial, Sprite,
};
use crate::game::Events;

use input::InputActions;

pub mod input;
pub mod npc;
pub mod physics;
pub mod sprite;

/// Game state.
pub struct State {
    pub world: World,
    pub player: Entity,
    pub actions: InputActions,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();
        let player = world.spawn((
            Player,
            Spatial::new(0, 0),
            Controller::default(),
            CoyoteTime::default(),
            Physics::new(0, 0),
            Collision::new((0, 0), (10, 14)),
            Sprite::Test(0),
            Animation::default(),
        ));
        debug!("player entity generated ({})", player.id());

        world.spawn((
            Spatial::new(0, 0),
            Physics::new(0, 0),
            Controller::default(),
            Collision::new((0, 0), (10, 14)),
            Sprite::Test(2),
            Animation::default(),
        ));

        world.spawn((
            Spatial::new(-100, -113),
            Sprite::Rat(0),
            Animation::default(),
        ));

        let actions = InputActions {
            jump: VirtualKeyCode::Space,
            move_left: VirtualKeyCode::A,
            move_right: VirtualKeyCode::D,
        };

        State {
            world,
            player,
            actions,
        }
    }

    pub fn update(&mut self, tick_rate: Duration, events: &mut Events) {
        self.event_npc_static_collision(events.physics.read());

        self.process_npcs(events);
        self.process_physics(events);
        self.process_sprites(tick_rate);
    }
}
