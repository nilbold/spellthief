use std::time::Duration;

use crate::component::{Animation, Sprite};
use crate::game::State;

impl State {
    pub fn process_sprites(&mut self, tick_rate: Duration) {
        for (_id, (sprite, anim)) in self.world.query_mut::<(&mut Sprite, &mut Animation)>() {
            anim.tick += tick_rate;
            if anim.tick < anim.next {
                continue;
            }

            let delay;
            (*sprite, delay) = sprite.next();

            anim.tick -= anim.next;
            anim.next = delay;
        }
    }
}
