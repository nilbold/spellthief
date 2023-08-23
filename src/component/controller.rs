#[allow(dead_code)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum MoveDirection {
    #[default]
    Right,
    Left,
}

impl MoveDirection {
    #[allow(dead_code)]
    pub fn is_right(&self) -> bool {
        *self == MoveDirection::Right
    }

    #[allow(dead_code)]
    pub fn is_left(&self) -> bool {
        *self == MoveDirection::Left
    }
}

/// Entity Controller, can be Player or AI driven.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Controller {
    pub locked: bool,
    pub jumping: bool,
    pub moving: bool,
    pub pre_jump: u32,
    pub post_jump: u32,
    pub direction: MoveDirection,
}

impl Controller {
    /// Reset controller state, generally at the end of the game loop.
    pub fn reset(&mut self) {
        self.jumping = false;
        self.moving = false;

        if self.pre_jump > 0 {
            self.pre_jump -= 1;
        }
        if self.post_jump > 0 {
            self.post_jump -= 1;
        }
    }

    pub fn face_left(&mut self) {
        self.direction = MoveDirection::Left;
    }

    pub fn face_right(&mut self) {
        self.direction = MoveDirection::Right;
    }
}
