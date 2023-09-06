use hecs::{Entity, World};
use log::debug;

use winit::event::VirtualKeyCode;

use crate::component::{Collision, Controller, CoyoteTime, Physics, Player, Spatial};

use input::InputActions;

pub mod input;
//pub mod npcs;
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
            CoyoteTime::default(),
            Physics::new(0, 0),
            Collision::new((0, 0), (10, 14)),
        ));
        debug!("player entity generated ({})", player.id());

        world.spawn((
            Spatial::new(0, 0),
            Physics::new(0, 0),
            Controller::default(),
            Collision::new((0, 0), (10, 14)),
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

    pub fn update(&mut self) {
        //self.process_npcs();
        self.process_physics();
    }
}
