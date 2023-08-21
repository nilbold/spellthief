use std::time::Duration;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use crate::component::{Collision, Spatial};
use crate::game::State;

pub use sprite::Sprite;
pub use surface::Surface;

pub mod shapes;
pub mod sprite;
pub mod surface;

/// Manages render state and drawing.
pub struct Renderer {
    width: u32,
    height: u32,
    offset: (i32, i32),
    pixels: Pixels,
}

impl Renderer {
    pub fn new(window: &Window, width: u32, height: u32) -> Result<Self> {
        let pixels = {
            let size = window.inner_size();
            let surface = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(width, height, surface)?
        };

        let offset = (width as i32 / 2, height as i32 / 2);

        Ok(Renderer {
            width,
            height,
            offset,
            pixels,
        })
    }

    #[allow(dead_code)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[allow(dead_code)]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Render the current game state to the screen.
    pub fn draw_world(&mut self, state: &State, _lag: Duration) -> Result<()> {
        let frame = self.pixels.frame_mut();

        // texture format for the render frame is assumed to be RGBA8
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i32;
            let y = (i / self.width as usize) as i32;

            let r = (x * 64 / self.width as i32) as u8;
            let g = (y * 64 / self.height as i32) as u8;
            let rgba = [r, g, 64 - r, 0xff];
            pixel.copy_from_slice(&rgba);
        }

        for (_entity, pos) in state.world.query::<&Spatial>().iter() {
            let pos = apply_camera(pos.screen(), self.offset, Some(self.height as i32));
            let surface = Surface::new(frame, self.width, self.height);

            Sprite::Test(0).blit(surface, pos);
        }

        // lets draw collision shapes, for testing
        for (_entity, (pos, coll)) in state.world.query::<(&Spatial, &Collision)>().iter() {
            let (min, max) = {
                let pos = apply_camera(
                    (pos + coll.bounds.pos).screen(),
                    self.offset,
                    Some(self.height as i32),
                );

                let dim = coll.bounds.dim.screen();
                (
                    (pos.0 - dim.0, pos.1 - dim.1),
                    (pos.0 + dim.0, pos.1 + dim.1),
                )
            };
            let surface = Surface::new(frame, self.width, self.height);

            shapes::corners(surface, min, max, [0x00, 0xff, 0xff, 0xff]);
        }

        self.pixels.render()?;
        Ok(())
    }

    /// Resize the render surface (window).
    ///
    /// This leaves the internal framebuffer its original size.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.pixels.resize_surface(width, height)?;
        Ok(())
    }
}

fn apply_camera(pos: (i32, i32), offset: (i32, i32), y_flip: Option<i32>) -> (i32, i32) {
    let (x, mut y) = (pos.0 + offset.0, pos.1 + offset.1);
    if let Some(y_flip) = y_flip {
        y = y_flip - y;
    }
    (x, y)
}
