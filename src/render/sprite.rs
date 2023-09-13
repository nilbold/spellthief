use crate::component::Sprite;
use crate::embed;
use crate::render::Surface;

impl Sprite {
    pub fn blit(&self, surface: &mut Surface, pos: (i32, i32), flip: bool) {
        let (sprite, frame) = match self {
            Self::Test(frame) => (embed::SPRITE_TEST, *frame),
        };

        assert!((frame as usize) < sprite.frames);

        let offset = if flip {
            (sprite.width as i32 - sprite.offset.0, sprite.offset.1)
        } else {
            (sprite.offset.0, sprite.offset.1)
        };

        let min = (pos.0 - offset.0, pos.1 - offset.1);
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
            let (mut temp_iter1, mut temp_iter2);
            let fo = frame as usize * sprite.height;

            let row_iter: &mut dyn Iterator<Item = _> = if flip {
                temp_iter1 = sprite.row(line + fo).rev().skip(clip.left).take(sw);
                &mut temp_iter1
            } else {
                temp_iter2 = sprite.row(line + fo).skip(clip.left).take(sw);
                &mut temp_iter2
            };

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
