#[derive(Debug, Default)]
pub struct Envelope {
    timer: u8,
}

impl Envelope {
    pub fn new() -> Self {
        Self { timer: 0 }
    }

    pub fn tick(&mut self, audenv: &yagber_memory::Audenv) -> i8 {
        if audenv.sweep_pace() == 0 {
            return 0;
        }
        self.timer = self.timer.wrapping_add(1);
        if self.timer >= audenv.sweep_pace() {
            self.timer = 0;
            match audenv.direction() {
                yagber_memory::EnvelopeDirection::Increase => 1,
                yagber_memory::EnvelopeDirection::Decrease => -1,
            }
        } else {
            0
        }
    }

    pub fn set_timer(&mut self, value: u8) {
        self.timer = value;
    }
}
