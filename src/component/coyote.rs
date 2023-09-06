/// Provides 'coyote time' to an Entity.
///
/// This adds some leeway in allowing jump inputs to just before landing on the
/// ground, or after running off of a ledge.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct CoyoteTime {
    pre: u32,
    //post: u32,
}

impl CoyoteTime {
    pub const FRAMES: u32 = 5;

    pub fn tick(&mut self) {
        if self.pre > 0 {
            self.pre -= 1;
        }
        //if self.post > 0 {
        //    self.post -= 1;
        //}
    }

    pub fn reset(&mut self) {
        self.pre = 0;
        //self.post = 0;
    }

    pub fn pre_jump(&self) -> bool {
        self.pre > 0
    }

    //pub fn post_jump(&self) -> bool {
    //    self.post > 0
    //}

    pub fn set_pre_jump(&mut self) {
        self.pre = Self::FRAMES;
    }

    //pub fn set_post_jump(&mut self) {
    //    self.post = Self::FRAMES;
    //}
}
