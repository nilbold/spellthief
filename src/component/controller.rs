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
    pub on_floor: bool,
    pub direction: MoveDirection,
}

impl Controller {
    /// Reset controller state, generally at the end of the game loop.
    pub fn reset(&mut self) {
        self.jumping = false;
        self.moving = false;
    }

    pub fn face_left(&mut self) {
        self.direction = MoveDirection::Left;
    }

    pub fn face_right(&mut self) {
        self.direction = MoveDirection::Right;
    }
}
