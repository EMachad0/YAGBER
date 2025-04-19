use std::time::Duration;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct Stopwatch {
    elapsed: Duration,
    is_paused: bool,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            elapsed: Duration::new(0, 0),
            is_paused: false,
        }
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn unpause(&mut self) {
        self.is_paused = false;
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::new(0, 0);
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }

    pub fn elapsed_secs_f64(&self) -> f64 {
        self.elapsed.as_secs_f64()
    }

    pub fn set_elapsed(&mut self, elapsed: Duration) {
        self.elapsed = elapsed;
    }

    pub fn tick(&mut self, delta: Duration) -> &Self {
        if !self.is_paused {
            self.elapsed += delta;
        }
        self
    }
}
