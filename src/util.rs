use std::time::{Duration, Instant};

/// Manages game tick rate.
pub struct TickRate {
    previous: Instant,
    lag: Duration,
    tick_rate: Duration,
}

impl TickRate {
    pub fn new(tick_rate: Duration) -> Self {
        TickRate {
            previous: Instant::now(),
            lag: Duration::ZERO,
            tick_rate,
        }
    }

    pub fn lag(&self) -> Duration {
        self.lag
    }

    pub fn tick_rate(&self) -> Duration {
        self.tick_rate
    }

    pub fn step(&mut self) {
        let current = Instant::now();
        let elapsed = current - self.previous;

        self.previous = current;
        self.lag += elapsed;
    }

    pub fn should_update(&mut self) -> bool {
        if self.lag < self.tick_rate {
            return false;
        }

        self.lag -= self.tick_rate;
        true
    }
}
