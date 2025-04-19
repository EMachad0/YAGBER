use std::time::{Duration, Instant};

use crate::time::TimeContext;

#[derive(Debug, Copy, Clone)]
pub struct WallClock {
    pub last_update: Instant,
}

impl WallClock {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.last_update.elapsed()
    }

    pub fn update(&mut self) {
        self.last_update = Instant::now();
    }
}

impl Default for WallClock {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeContext for WallClock {
    fn elapsed(&self) -> std::time::Duration {
        self.elapsed()
    }
}
