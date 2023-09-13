pub type Frame = u16;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Sprite {
    Test(Frame),
}

impl Sprite {
    pub fn frame(&self, frame: Frame) -> Self {
        match self {
            Self::Test(_) => Self::Test(frame),
        }
    }
}
