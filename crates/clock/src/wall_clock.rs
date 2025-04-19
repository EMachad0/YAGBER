use std::time::{Duration, Instant};

use crate::time::TimeContext;

#[derive(Debug, Copy, Clone)]
pub struct WallClock {
    pub created: Instant,
    pub last_update: Option<Instant>,
}

impl WallClock {
    pub fn new() -> Self {
        Self {
            created: Instant::now(),
            last_update: None,
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.last_update.map(|lu| lu.elapsed()).unwrap_or_default()
    }

    pub fn update(&mut self) {
        self.last_update = Some(Instant::now());
    }
}

impl Default for WallClock {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeContext for WallClock {
    fn delta(&self) -> std::time::Duration {
        self.elapsed()
    }

    fn update(&mut self) {
        self.update();
    }

    fn elapsed(&self) -> std::time::Duration {
        self.created.elapsed()
    }
}
