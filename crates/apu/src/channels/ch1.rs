#[derive(Debug, Default)]
pub struct Ch1 {
    period: u16,
    duty_step_counter: u8,
    length_counter: u8,
}

impl Ch1 {
    const DEFAULT_LENGTH_COUNTER: u8 = 64;

    pub fn new() -> Self {
        Self {
            period: 0,
            length_counter: 0,
            duty_step_counter: 0,
        }
    }

    pub fn trigger(&mut self, bus: &yagber_memory::Bus) {
        self.length_counter = self.get_initial_length_counter(bus);
        self.period = self.get_initial_period(bus);
    }

    pub fn tick(&mut self, bus: &yagber_memory::Bus) {
        self.period = self.period.wrapping_sub(1);
        if self.period != 0 {
            return;
        }
        self.period = self.get_initial_period(bus);

        let duty_step = self.duty_step(bus);
        // TODO: create sample

        self.duty_step_counter = (self.duty_step_counter + 1) % 8;
    }

    fn duty_step(&self, bus: &yagber_memory::Bus) -> bool {
        let aud_1_len = yagber_memory::Aud1Len::from_bus(bus);
        let wave_duty = aud_1_len.wave_duty();
        wave_duty.is_high(self.duty_step_counter)
    }

    pub fn decrement_length_counter(&mut self) -> u8 {
        self.length_counter = self.length_counter.wrapping_sub(1);
        self.length_counter
    }

    fn get_initial_period(&self, bus: &yagber_memory::Bus) -> u16 {
        let high = yagber_memory::Aud1High::from_bus(bus).period_high() as u16;
        let low = bus.read(yagber_memory::IOType::AUD1LOW.address()) as u16;
        let period = (high << 8) | low;
        0x800 - period
    }

    fn get_initial_length_counter(&self, bus: &yagber_memory::Bus) -> u8 {
        let current = self.length_counter;
        let initial = yagber_memory::Aud1Len::from_bus(bus).timer_length();
        if current != 0 {
            current
        } else if initial != 0 {
            initial
        } else {
            Self::DEFAULT_LENGTH_COUNTER
        }
    }
}
