use std::time::Duration;

use crate::component::sprite::{Frame, Sprite};

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum AnimationState {
    #[default]
    Idle,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Animation {
    pub tick: Duration,
    pub next: Duration,
    pub state: AnimationState,
}

impl Animation {
    pub fn new(state: AnimationState) -> Self {
        Animation {
            tick: Duration::ZERO,
            next: Duration::from_millis(200),
            state,
        }
    }
}

impl AnimationState {
    pub fn lookup_next(&self, sprite: Sprite) -> (Frame, u16) {
        match self {
            Self::Idle => match sprite {
                Sprite::Test(0) => (1, 200),
                Sprite::Test(1) => (2, 200),
                Sprite::Test(2) => (3, 200),
                Sprite::Test(3) => (0, 200),
                Sprite::Test(_) => (0, 200),
            },
        }
    }
}
