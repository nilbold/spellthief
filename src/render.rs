use std::time::Duration;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use crate::component::Spatial;
use crate::game::State;

/// Manages render state and drawing.
pub struct Renderer {
    width: u32,
    height: u32,
    pixels: Pixels,
}

impl Renderer {
    pub fn new(window: &Window, width: u32, height: u32) -> Result<Self> {
        let pixels = {
            let size = window.inner_size();
            let surface = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(width, height, surface)?
        };

        Ok(Renderer {
            width,
            height,
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
    pub fn draw(&mut self, state: &State, _lag: Duration) -> Result<()> {
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

        // just for testing, drawing a 32x32 square for each entity
        for (_id, pos) in state.world.query::<&Spatial>().iter() {
            let pos = pos.screen();
            for y in 0..32 {
                let i = (pos.x * 4 + pos.y * self.width as i32 * 4 + y * self.width as i32 * 4)
                    as usize;
                frame[i..i + 32 * 4].iter_mut().for_each(|p| *p = 0);
            }
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
