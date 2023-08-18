use std::time::Duration;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use crate::component::Spatial;
use crate::game::State;
use crate::math::Vector;

pub use sprite::Sprite;

pub mod sprite;

/// Manages render state and drawing.
pub struct Renderer {
    width: u32,
    height: u32,
    offset: Vector,
    pixels: Pixels,
}

impl Renderer {
    pub fn new(window: &Window, width: u32, height: u32) -> Result<Self> {
        let pixels = {
            let size = window.inner_size();
            let surface = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(width, height, surface)?
        };

        let offset = Vector::new(width as i32 / 2, height as i32 / 2);

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

            let r = (x * 255 / self.width as i32) as u8;
            let g = (y * 255 / self.height as i32) as u8;
            let rgba = [r, g, 0xff, 0xff];
            pixel.copy_from_slice(&rgba);
        }

        for (_id, pos) in state.world.query::<&Spatial>().iter() {
            let pos = {
                let mut pos = pos.screen() + self.offset;
                pos.y = self.height as i32 - pos.y;
                pos
            };

            Sprite::Test(0).blit(frame, self.width, self.height, pos.x, pos.y);
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
