use std::time::Duration;

use crate::embed::{self, EmbeddedSprite};

pub type Frame = u16;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Sprite {
    Test(Frame),
}

impl Sprite {
    /// Convienence method to retrieve embedded sprite data.
    pub fn data(self) -> (&'static EmbeddedSprite<'static>, Frame) {
        match self {
            Self::Test(frame) => (&embed::SPRITE_TEST, frame),
        }
    }

    /// Gets the next frame for an animated sprite.
    pub fn next(self) -> (Self, Duration) {
        let (data, mut frame) = self.data();

        frame += 1;
        if frame >= data.frames {
            frame = 0;
        }

        let sprite = match self {
            Self::Test(_) => Self::Test(frame),
        };

        (sprite, Duration::from_millis(data.delay(frame) as u64))
    }
}
