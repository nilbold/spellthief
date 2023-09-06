use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper as Input;

use crate::component::{Controller, CoyoteTime};
use crate::game::State;

pub struct InputActions {
    pub jump: VirtualKeyCode,
    pub move_left: VirtualKeyCode,
    pub move_right: VirtualKeyCode,
}

impl State {
    pub fn input(&mut self, input: &mut Input) {
        let (conn, yote) = self
            .world
            .query_one_mut::<(&mut Controller, &mut CoyoteTime)>(self.player)
            .expect("world.get(player)");

        if conn.locked {
            return;
        }

        if input.key_pressed(self.actions.jump) {
            if conn.on_floor {
                conn.jumping = true;
            } else {
                yote.set_pre_jump();
            }
        }
        if input.key_held(self.actions.jump) && !conn.on_floor {
            conn.jumping = true;
        }

        let (l, r) = {
            (
                input.key_held(self.actions.move_left),
                input.key_held(self.actions.move_right),
            )
        };

        if l ^ r {
            conn.moving = true;
            if l {
                conn.face_left();
            } else {
                conn.face_right();
            }
        }
    }
}
