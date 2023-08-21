use crate::embed;
use crate::render::Surface;

pub type Frame = u16;

pub enum Sprite {
    Test(Frame),
}

impl Sprite {
    pub fn blit(&self, surface: Surface, pos: (i32, i32)) {
        let sprite = match self {
            Self::Test(_) => embed::SPRITE_TEST,
        };

        let min = (pos.0 - sprite.offset.0, pos.1 - sprite.offset.1);
        let max = (min.0 + sprite.width as i32, min.1 + sprite.height as i32);

        let clip = {
            if let Some(c) = surface.clip(min, max) {
                c
            } else {
                // sprite is off screen, no use trying to blit
                return;
            }
        };

        let x: usize = (min.0 + clip.left as i32) as usize;
        let y: usize = (min.1 + clip.top as i32) as usize;
        let fw: usize = surface.width as usize;
        let sw: usize = sprite.width - clip.right;

        // copy one line at a time to the buffer
        for line in clip.top..sprite.height - clip.bottom {
            let row_iter = sprite.row(line).skip(clip.left).take(sw);

            let i = (x + (y + line - clip.top) * fw) * 4;
            for (pixel, c) in surface.buffer[i..i + sw * 4].chunks_mut(4).zip(row_iter) {
                if c > 0 {
                    // TODO palette, for now every non zero entry is white
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
            }
        }
    }
}
