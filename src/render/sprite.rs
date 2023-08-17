use crate::embed;

pub type Frame = u16;

pub enum Sprite {
    Test(Frame),
}

impl Sprite {
    pub fn blit(&self, frame: &mut [u8], frame_width: u32, frame_height: u32, x: i32, y: i32) {
        let sprite = match self {
            Self::Test(_) => embed::SPRITE_TEST,
        };

        // we'll need to clip the sprite if it's partially off screen
        let (cl, ct, cr, cb) = {
            let cl: usize = (0 - x).max(0).try_into().unwrap();
            let cr: usize = (x - frame_width as i32 + sprite.width as i32)
                .max(0)
                .try_into()
                .unwrap();
            let ct: usize = (0 - y).max(0).try_into().unwrap();
            let cb: usize = (y - frame_height as i32 + sprite.height as i32)
                .max(0)
                .try_into()
                .unwrap();

            // check if the sprite is fully off the screen
            if cl > sprite.width || cr > sprite.width || ct > sprite.height || cb > sprite.height {
                return;
            }

            (cl, ct, cr, cb)
        };

        let x: usize = (x + cl as i32).try_into().unwrap();
        let y: usize = (y + ct as i32).try_into().unwrap();
        let fw: usize = frame_width.try_into().unwrap();
        let sw: usize = sprite.width - cr;

        // copy one line at a time to the buffer
        for line in ct..sprite.height - cb {
            let row_iter = sprite.row(line).skip(cl).take(sw);

            let i = (x + (y + line - ct) * fw) * 4;
            for (pixel, c) in frame[i..i + sw * 4].chunks_mut(4).zip(row_iter) {
                if c > 0 {
                    // TODO palette, for now every non zero entry is white
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
            }
        }
    }
}
