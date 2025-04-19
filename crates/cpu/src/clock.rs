use clock::{Timer, WallClock};

/// CPU clock module
/// Duration of 1 M-cycle
pub struct CpuClock {
    wall_clock: WallClock,
    timer: Timer,
}

impl CpuClock {
    const CPU_FREQ_HZ: u64 = 4_194_304;
    const M_CYCLE_DURATION: u64 = (1_000_000_000 * 4) / Self::CPU_FREQ_HZ; // ≃ 953 ns

    pub fn new() -> Self {
        Self {
            wall_clock: WallClock::new(),
            timer: Timer::new(
                std::time::Duration::from_nanos(Self::M_CYCLE_DURATION),
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

impl Default for CpuClock {
    fn default() -> Self {
        Self::new()
    }
}
