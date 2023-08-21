pub struct EdgeClip {
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize,
}

pub struct Surface<'a> {
    pub buffer: &'a mut [u8],
    pub width: u32,
    pub height: u32,
}

impl<'a> Surface<'a> {
    pub fn new(buffer: &'a mut [u8], width: u32, height: u32) -> Self {
        Surface {
            buffer,
            width,
            height,
        }
    }

    /// Edge clipping.
    ///
    /// Given the min and max defining a rectangular area, determine clipping.
    /// Returns None if an area is fully beyond the surface edge.
    pub fn clip(&self, min: (i32, i32), max: (i32, i32)) -> Option<EdgeClip> {
        let w: usize = (max.0 - min.0).try_into().expect("clip max.0 - min.0");
        let h: usize = (max.1 - min.1).try_into().expect("clip max.1 - min.1");

        let left = (0 - min.0).max(0) as usize;
        let right = (min.0 - self.width as i32 + w as i32).max(0) as usize;
        let top = (0 - min.1).max(0) as usize;
        let bottom = (min.1 - self.height as i32 + h as i32).max(0) as usize;

        if left > w || right > w || top > h || bottom > h {
            return None;
        }

        Some(EdgeClip {
            left,
            right,
            top,
            bottom,
        })
    }
}
