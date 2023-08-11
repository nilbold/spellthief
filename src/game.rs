use std::time::{Duration, Instant};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper as Input;

use anyhow::Result;
use hecs::{Entity, World};

use crate::component::Spatial;

/// Game state.
pub struct State {
    pub world: World,
    pub player: Entity,
}

/// Manages game tick rate.
struct TickRate {
    previous: Instant,
    lag: Duration,
    tick_rate: Duration,
}

impl TickRate {
    fn new(tick_rate: Duration) -> Self {
        TickRate {
            previous: Instant::now(),
            lag: Duration::ZERO,
            tick_rate,
        }
    }

    fn step(&mut self) {
        let current = Instant::now();
        let elapsed = current - self.previous;

        self.previous = current;
        self.lag += elapsed;
    }

    fn should_update(&mut self) -> bool {
        if self.lag < self.tick_rate {
            return false;
        }

        self.lag -= self.tick_rate;
        true
    }
}

/// Enters the main game loop.
pub fn main_loop() -> Result<()> {
    let event_loop = EventLoop::new();
    let _window = {
        let size = LogicalSize::new(320.0, 240.0);
        let scaled = LogicalSize::new(size.width * 3.0, size.height * 3.0);
        WindowBuilder::new()
            .with_title("spellthief")
            .with_inner_size(scaled)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut input = Input::new();

    let mut world = World::new();
    let player = world.spawn((Spatial::default(),));

    let mut state = State { world, player };

    let mut tick = TickRate::new(Duration::from_millis(20));
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            tick.step();
            while tick.should_update() {
                update(&mut state, &mut input);
            }

            render(&mut state, tick.lag);

            if input.close_requested() {
                control_flow.set_exit();
            }
        }
    });
}

/// Update a single game frame.
fn update(_state: &mut State, _input: &mut Input) {}

/// Render the current game state to the screen.
fn render(_state: &mut State, _lag: Duration) {}
