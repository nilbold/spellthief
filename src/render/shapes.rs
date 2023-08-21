use crate::render::Surface;

pub fn corners(surface: Surface, min: (i32, i32), max: (i32, i32), color: [u8; 4]) {
    let clip = {
        if let Some(c) = surface.clip(min, max) {
            c
        } else {
            return;
        }
    };

    let w: usize = (max.0 - min.0).try_into().expect("clip max.0 - min.0");
    let h: usize = (max.1 - min.1).try_into().expect("clip max.1 - min.1");

    // 2x2 is the minimum size we'll draw
    if w < 2 || h < 2 {
        return;
    }

    let x: usize = (min.0 + clip.left as i32) as usize;
    let y: usize = (min.1 + clip.top as i32) as usize;
    let fw: usize = surface.width as usize;

    if clip.left + clip.top == 0 {
        let i = (x + (y - clip.top) * fw) * 4;
        surface.buffer[i..i + 4].copy_from_slice(&color);
    }
    if clip.right + clip.top == 0 {
        let i = (x + (w - 1) - clip.left + (y - clip.top) * fw) * 4;
        surface.buffer[i..i + 4].copy_from_slice(&color);
    }
    if clip.left + clip.bottom == 0 {
        let i = (x + (y + (h - 1) - clip.top) * fw) * 4;
        surface.buffer[i..i + 4].copy_from_slice(&color);
    }
    if clip.right + clip.bottom == 0 {
        let i = (x + (w - 1) - clip.left + (y + (h - 1) - clip.top) * fw) * 4;
        surface.buffer[i..i + 4].copy_from_slice(&color);
    }
}
