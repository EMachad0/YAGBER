use std::time::Duration;

pub trait TimeContext: Default + Copy + Clone {
    fn delta(&self) -> Duration;
    fn update(&mut self);
    fn elapsed(&self) -> Duration;
}

#[derive(Debug, Copy, Clone)]
pub struct Time<T: TimeContext> {
    context: T,
    wrap_period: Duration,
    delta: Duration,
    delta_secs: f32,
    delta_secs_f64: f64,
    elapsed: Duration,
    elapsed_secs: f32,
    elapsed_secs_f64: f64,
    elapsed_wrapped: Duration,
    elapsed_secs_wrapped: f32,
    elapsed_secs_wrapped_f64: f64,
}

impl<T: TimeContext> Time<T> {
    const DEFAULT_WRAP_PERIOD: Duration = Duration::from_secs(3600); // 1 hour

    pub fn new(context: T) -> Self {
        Self {
            context,
            ..Default::default()
        }
    }

    pub fn advance(&mut self) {
        let delta = self.context.delta();
        self.context.update();
        self.advance_by(delta);
    }

    pub fn advance_by(&mut self, delta: Duration) {
        self.delta = delta;
        self.delta_secs = delta.as_secs_f32();
        self.delta_secs_f64 = delta.as_secs_f64();
        self.elapsed += delta;
        self.elapsed_secs = self.elapsed.as_secs_f32();
        self.elapsed_secs_f64 = self.elapsed.as_secs_f64();
        self.elapsed_wrapped = duration_rem(self.elapsed, self.wrap_period);
        self.elapsed_secs_wrapped = self.elapsed_wrapped.as_secs_f32();
        self.elapsed_secs_wrapped_f64 = self.elapsed_wrapped.as_secs_f64();
    }

    pub fn set_wrap_period(&mut self, wrap_period: Duration) {
        self.wrap_period = wrap_period;
    }

    pub fn wrap_period(&self) -> Duration {
        self.wrap_period
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn delta_secs(&self) -> f32 {
        self.delta_secs
    }

    pub fn delta_secs_f64(&self) -> f64 {
        self.delta_secs_f64
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed_secs
    }

    pub fn elapsed_secs_f64(&self) -> f64 {
        self.elapsed_secs_f64
    }

    pub fn elapsed_wrapped(&self) -> Duration {
        self.elapsed_wrapped
    }

    pub fn elapsed_secs_wrapped(&self) -> f32 {
        self.elapsed_secs_wrapped
    }

    pub fn elapsed_secs_wrapped_f64(&self) -> f64 {
        self.elapsed_secs_wrapped_f64
    }
}

impl<T: TimeContext> Default for Time<T> {
    fn default() -> Self {
        Self {
            context: Default::default(),
            wrap_period: Self::DEFAULT_WRAP_PERIOD,
            delta: Duration::ZERO,
            delta_secs: 0.0,
            delta_secs_f64: 0.0,
            elapsed: Duration::ZERO,
            elapsed_secs: 0.0,
            elapsed_secs_f64: 0.0,
            elapsed_wrapped: Duration::ZERO,
            elapsed_secs_wrapped: 0.0,
            elapsed_secs_wrapped_f64: 0.0,
        }
    }
}

fn duration_rem(dividend: Duration, divisor: Duration) -> Duration {
    // `Duration` does not have a built-in modulo operation
    let quotient = (dividend.as_nanos() / divisor.as_nanos()) as u32;
    dividend - (quotient * divisor)
}
