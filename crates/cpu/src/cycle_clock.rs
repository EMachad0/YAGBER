use yagber_clock::{Timer, TimerMode};

/// CPU clock module
/// Duration of 1 M-cycle
#[derive(Debug, Clone, Copy)]
pub struct CycleClock {
    timer: Timer,
}

impl CycleClock {
    const CPU_FREQ_HZ: u64 = 4_194_304; // 4.194304 MHz
    const M_CYCLE_DURATION: u64 = (1_000_000_000 * 4) / Self::CPU_FREQ_HZ; // ≃ 953 ns

    pub fn new() -> Self {
        Self {
            timer: Timer::new(
                std::time::Duration::from_nanos(Self::M_CYCLE_DURATION),
                TimerMode::Repeating,
            ),
        }
    }

    pub fn tick(&mut self, delta: std::time::Duration) {
        self.timer.tick(delta);
    }

    pub fn times_finished_this_tick(&self) -> u32 {
        self.timer.times_finished_this_tick()
    }
}

impl Default for CycleClock {
    fn default() -> Self {
        Self::new()
    }
}
