use std::time::Duration;

use anyhow::Result;
use hecs::{Entity, World};
use log::{debug, error};
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper as Input;

use crate::component::{Physics, Player, Spatial};
use crate::util::TickRate;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

/// Game state.
pub struct State {
    pub world: World,
    pub player: Entity,
}

/// Enters the main game loop.
pub fn main_loop() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled = LogicalSize::new(size.width * 3.0, size.height * 3.0);
        WindowBuilder::new()
            .with_title("spellthief")
            .with_inner_size(scaled)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut input = Input::new();

    let mut pixels = {
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface)?
    };

    let mut world = World::new();

    let player = world.spawn((Player, Spatial::new(0, 0), Physics::default()));
    debug!("player entity generated ({})", player.id());

    let mut state = State { world, player };

    let mut tick = TickRate::new(Duration::from_millis(20));
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            render(pixels.frame_mut(), &mut state, tick.lag());

            if let Err(why) = pixels.render() {
                error!("pixels.render failed: {why}");
                control_flow.set_exit();
                return;
            }
        }

        if input.update(&event) {
            tick.step();

            if let Some(size) = input.window_resized() {
                if let Err(why) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface failed: {why}");
                    control_flow.set_exit();
                    return;
                }
            }

            if input.close_requested() {
                debug!("window close requested");
                control_flow.set_exit();
                return;
            }

            while tick.should_update() {
                update(&mut state, &mut input);
            }

            window.request_redraw();
        }
    });
}

/// Update a single game frame.
fn update(_state: &mut State, _input: &mut Input) {}

/// Render the current game state to the screen.
///
/// Texture format for the render frame is assumed to be RGBA8.
fn render(frame: &mut [u8], state: &mut State, _lag: Duration) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as i32;
        let y = (i / WIDTH as usize) as i32;

        let r = (x * 255 / WIDTH as i32) as u8;
        let g = (y * 255 / HEIGHT as i32) as u8;
        let rgba = [r, g, 0xff, 0xff];
        pixel.copy_from_slice(&rgba);
    }

    // just for testing, drawing a 32x32 square for each entity
    for (_id, pos) in state.world.query::<&Spatial>().iter() {
        let pos = pos.screen();
        for y in 0..32 {
            let i = (pos.x * 4 + pos.y * WIDTH as i32 * 4 + y * WIDTH as i32 * 4) as usize;
            frame[i..i + 32 * 4].iter_mut().for_each(|p| *p = 0);
        }
    }
}
