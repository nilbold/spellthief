use std::time::Duration;

use anyhow::Result;
use log::{debug, error};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper as Input;

use crate::render::Renderer;
use crate::util::TickRate;

pub use state::State;

mod state;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

/// Enters the main game loop.
pub fn main_loop() -> Result<()> {
    let (event_loop, window) = init_window("spellthief", WIDTH, HEIGHT, 3);
    let mut input = Input::new();

    let mut render = Renderer::new(&window, WIDTH, HEIGHT)?;
    let mut state = State::new();

    let mut tick = TickRate::new(Duration::from_millis(20));
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if let Err(why) = render.draw_world(&state, tick.lag()) {
                error!("render.draw failed: {why}");
                control_flow.set_exit();
                return;
            }
        }

        if input.update(&event) {
            tick.step();

            if let Some(size) = input.window_resized() {
                if let Err(why) = render.resize(size.width, size.height) {
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

fn init_window(title: &str, width: u32, height: u32, scaled: u32) -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        let scaled = LogicalSize::new(size.width * scaled as f64, size.height * scaled as f64);
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(scaled)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    (event_loop, window)
}
