use std::time::Duration;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Animation {
    pub tick: Duration,
    pub next: Duration,
}
