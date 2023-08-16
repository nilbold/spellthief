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

        if x < 0
            || y < 0
            || x >= frame_width as i32 - sprite.width as i32
            || y >= frame_height as i32 - sprite.height as i32
        {
            return;
        }

        let x: usize = x.try_into().unwrap();
        let fw: usize = frame_width.try_into().unwrap();

        let mut buffer = Vec::new();
        buffer.reserve_exact(sprite.width);

        // copy one line at a time to the buffer
        for line in 0..sprite.height {
            let i = line * sprite.width / 4;
            for b in sprite.data[i..i + sprite.width / 4].iter() {
                for i in (0..4).rev() {
                    // sprite data 2 bits per pixel
                    buffer.push(b >> (i * 2) & 3);
                }
            }

            let i = x * 4 + line * fw * 4;
            for (pixel, &c) in frame[i..i + sprite.width * 4]
                .chunks_mut(4)
                .zip(buffer.iter())
            {
                if c > 0 {
                    // TODO palette, for now every non zero entry is white
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
            }
            buffer.clear();
        }
    }
}
