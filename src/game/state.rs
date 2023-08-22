use hecs::{Entity, World};
use log::debug;

use winit::event::VirtualKeyCode;

use crate::component::{Collision, Controller, Physics, Player, Spatial};
use crate::math::Vector;

use input::InputActions;

pub mod input;
pub mod physics;

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
            Physics::new(0, 0),
            Collision::new((0, 0), (10 << Vector::F, 14 << Vector::F)),
        ));
        debug!("player entity generated ({})", player.id());

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
}
