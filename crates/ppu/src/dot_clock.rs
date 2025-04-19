use clock::{Timer, WallClock};

/// CPU clock module
/// Duration of 1 Dot
pub struct DotClock {
    wall_clock: WallClock,
    timer: Timer,
}

impl DotClock {
    const CPU_FREQ_HZ: u64 = 4_194_304; // 4.194304 MHz
    const DOT_DURATION: u64 = 1_000_000_000 / Self::CPU_FREQ_HZ; // ≃ 239 ns

    pub fn new() -> Self {
        Self {
            wall_clock: WallClock::new(),
            timer: Timer::new(
                std::time::Duration::from_nanos(Self::DOT_DURATION),
                clock::TimerMode::Repeating,
            ),
        }
    }

    pub fn tick(&mut self) {
        let elapsed = self.wall_clock.elapsed();
        self.timer.tick(elapsed);
    }

    pub fn times_finished_this_tick(&self) -> u32 {
        self.timer.times_finished_this_tick()
    }
}

impl Default for DotClock {
    fn default() -> Self {
        Self::new()
    }
}
