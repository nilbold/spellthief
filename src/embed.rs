//! Embedded game data.

pub struct EmbeddedSprite<'a> {
    pub width: u32,
    pub height: u32,
    pub offset: (i32, i32),
    pub data: &'a [u8],
    pub palette: &'a [[u8; 4]],
    pub frames: u16,
    pub default_delay: u16,
    pub anim: &'a [u16],
}

impl EmbeddedSprite<'_> {
    pub fn row(&self, r: usize) -> EmbeddedSpriteIter {
        EmbeddedSpriteIter {
            sprite: self,
            left: 0,
            right: self.width as usize,
            row: r,
        }
    }

    /// Gets the animation delay for the current frame.
    pub fn delay(&self, frame: u16) -> u16 {
        match self.anim.get(frame as usize) {
            Some(&delay) => delay,
            None => self.default_delay,
        }
    }
}

pub struct EmbeddedSpriteIter<'a> {
    sprite: &'a EmbeddedSprite<'a>,
    left: usize,
    right: usize,
    row: usize,
}

impl<'a> EmbeddedSpriteIter<'a> {
    fn index(&self, pos: usize) -> (usize, usize) {
        (
            (pos / 4) + self.row * (self.sprite.width as usize / 4),
            pos % 4,
        )
    }
}

impl<'a> Iterator for EmbeddedSpriteIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left >= self.right {
            return None;
        }

        let (i, r) = self.index(self.left);
        self.left += 1;

        Some(self.sprite.data[i] >> ((3 - r) * 2) & 3)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.sprite.width as usize, Some(self.sprite.width as usize))
    }
}

impl<'a> DoubleEndedIterator for EmbeddedSpriteIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.right <= self.left {
            return None;
        }

        let (i, r) = self.index(self.right - 1);
        self.right -= 1;

        Some(self.sprite.data[i] >> ((3 - r) * 2) & 3)
    }
}

impl<'a> ExactSizeIterator for EmbeddedSpriteIter<'a> {}

pub const PALETTE_WHITE: &[[u8; 4]] = &[
    [0x00, 0x00, 0x00, 0x00],
    [0xff, 0xff, 0xff, 0xff],
    [0xff, 0xff, 0xff, 0xff],
    [0xff, 0xff, 0xff, 0xff],
];

pub const PALETTE_RAT: &[[u8; 4]] = &[
    [0x00, 0x00, 0x00, 0x00],
    [0x8f, 0x56, 0x3b, 0xff],
    [0xee, 0xc3, 0x9a, 0xff],
    [0x22, 0x20, 0x34, 0xff],
];

pub const SPRITE_TEST: EmbeddedSprite = EmbeddedSprite {
    width: 20,
    height: 26,
    offset: (11, 13),
    data: include_bytes!("../embed/kobold_2i.bin"),
    palette: PALETTE_WHITE,
    frames: 4,
    default_delay: 200,
    anim: &[],
};

pub const SPRITE_RAT: EmbeddedSprite = EmbeddedSprite {
    width: 16,
    height: 14,
    offset: (8, 7),
    data: include_bytes!("../embed/rat_2i.bin"),
    palette: PALETTE_RAT,
    frames: 4,
    default_delay: 250,
    anim: &[],
};
