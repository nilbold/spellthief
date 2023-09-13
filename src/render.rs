use std::time::Duration;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use crate::component::{Collision, Controller, Spatial, Sprite};
use crate::game::State;

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
        let mut surface = Surface::new(self.pixels.frame_mut(), self.width, self.height);
        surface.clear([0x20, 0x20, 0x30, 0xff]);

        for (_entity, (pos, sprite, conn)) in state
            .world
            .query::<(&Spatial, &Sprite, Option<&Controller>)>()
            .iter()
        {
            let pos = apply_camera(pos.screen(), self.offset, Some(self.height as i32));
            let flip = conn.map_or(false, |c| c.direction.is_left());

            sprite.blit(&mut surface, pos, flip);
            //Sprite::Test(1).blit(&mut surface, pos, flip);
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
            shapes::corners(&mut surface, min, max, [0x00, 0xff, 0xff, 0xff]);
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
