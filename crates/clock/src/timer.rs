use std::time::Duration;

use crate::stopwatch::Stopwatch;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TimerMode {
    /// Run once and stop.
    #[default]
    Once,
    /// Reset when finished.
    Repeating,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    stopwatch: Stopwatch,
    duration: Duration,
    mode: TimerMode,
    finished: bool,
    times_finished_this_tick: u32,
}

impl Timer {
    pub fn new(duration: Duration, mode: TimerMode) -> Self {
        Self {
            duration,
            mode,
            ..Default::default()
        }
    }

    pub fn from_secs(duration: u64, mode: TimerMode) -> Self {
        Self::new(Duration::from_secs(duration), mode)
    }

    /// Indicates if the timer has ever finished.
    /// Remains true after the first finish for repeating timers.
    /// This is reset when the timer is reset.
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Indicates if the timer has finished this tick.
    /// This is reset when the timer is reset.
    pub fn just_finished(&self) -> bool {
        self.times_finished_this_tick > 0
    }

    pub fn elapsed(&self) -> Duration {
        self.stopwatch.elapsed()
    }

    pub fn elapsed_secs(&self) -> f32 {
        self.stopwatch.elapsed_secs()
    }

    pub fn elapsed_secs_f64(&self) -> f64 {
        self.stopwatch.elapsed_secs_f64()
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn pause(&mut self) {
        self.stopwatch.pause();
    }

    pub fn unpause(&mut self) {
        self.stopwatch.unpause();
    }

    pub fn reset(&mut self) {
        self.stopwatch.reset();
        self.finished = false;
        self.times_finished_this_tick = 0;
    }

    pub fn paused(&self) -> bool {
        self.stopwatch.is_paused()
    }

    pub fn fraction(&self) -> f32 {
        if self.duration.as_secs() == 0 {
            return 0.0;
        }
        self.elapsed().as_secs_f32() / self.duration.as_secs_f32()
    }

    pub fn remaining(&self) -> Duration {
        self.duration - self.elapsed()
    }

    pub fn remaining_secs(&self) -> f32 {
        self.remaining().as_secs_f32()
    }

    pub fn fraction_remaining(&self) -> f32 {
        1.0 - self.fraction()
    }

    pub fn times_finished_this_tick(&self) -> u32 {
        self.times_finished_this_tick
    }

    pub fn tick(&mut self, delta: Duration) -> &Self {
        self.times_finished_this_tick = 0;

        if self.paused() {
            return self;
        }

        self.stopwatch.tick(delta);

        if self.stopwatch.elapsed() >= self.duration {
            self.finished = true;

            match self.mode {
                TimerMode::Once => {
                    self.times_finished_this_tick = 1;
                    self.stopwatch.set_elapsed(self.duration);
                }
                TimerMode::Repeating => {
                    self.times_finished_this_tick = self
                        .elapsed()
                        .as_nanos()
                        .checked_div(self.duration().as_nanos())
                        .map_or(u32::MAX, |x| x as u32);
                    self.stopwatch.set_elapsed(Duration::from_nanos(
                        self.elapsed()
                            .as_nanos()
                            .checked_rem(self.duration().as_nanos())
                            .map_or(0, |x| x as u64),
                    ));
                }
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_non_repeating() {
        let mut timer = Timer::new(Duration::from_secs(1), TimerMode::Once);
        assert_eq!(timer.finished(), false);
        assert_eq!(timer.just_finished(), false);

        timer.tick(Duration::from_millis(500));
        assert_eq!(timer.finished(), false);
        assert_eq!(timer.just_finished(), false);

        timer.tick(Duration::from_millis(600));
        assert_eq!(timer.finished(), true);
        assert_eq!(timer.just_finished(), true);

        timer.reset();
        assert_eq!(timer.finished(), false);
    }

    #[test]
    fn test_timer_repeating() {
        let mut timer = Timer::new(Duration::from_secs(1), TimerMode::Repeating);
        assert_eq!(timer.finished(), false);
        assert_eq!(timer.just_finished(), false);

        timer.tick(Duration::from_millis(500));
        assert_eq!(timer.finished(), false);
        assert_eq!(timer.just_finished(), false);

        timer.tick(Duration::from_millis(600));
        assert_eq!(timer.finished(), true);
        assert_eq!(timer.just_finished(), true);

        timer.tick(Duration::from_millis(600));
        assert_eq!(timer.finished(), true);
        assert_eq!(timer.just_finished(), false);

        timer.tick(Duration::from_millis(600));
        assert_eq!(timer.finished(), true);
        assert_eq!(timer.just_finished(), true);

        timer.reset();
        assert_eq!(timer.finished(), false);
        assert_eq!(timer.just_finished(), false);
    }
}
