use crate::{Apu, channels::Envelope};

#[derive(Debug, Default)]
pub struct Ch1 {
    period: u16,
    duty_step_counter: u8,
    length_counter: u8,
    volume: u8,
    pub envelope: Envelope,
    pub sample: u8,
}

impl Ch1 {
    const DEFAULT_LENGTH_COUNTER: u8 = 64;

    pub fn new() -> Self {
        Self {
            period: 0,
            length_counter: 0,
            duty_step_counter: 0,
            volume: 0,
            envelope: Envelope::new(),
            sample: 0,
        }
    }

    pub fn trigger(&mut self, bus: &yagber_memory::Bus) {
        self.length_counter = self.get_initial_length_counter(bus);
        self.period = self.get_initial_period(bus);
        self.volume = self.get_initial_volume(bus);
        self.envelope.set_timer(0);
    }

    pub fn tick(&mut self, bus: &yagber_memory::Bus) {
        self.period = self.period.wrapping_sub(1);
        if self.period != 0 {
            return;
        }
        self.period = self.get_initial_period(bus);

        let duty_step = self.duty_step(bus);

        self.duty_step_counter = (self.duty_step_counter + 1) % 8;

        self.sample = duty_step * self.volume;
    }

    fn duty_step(&self, bus: &yagber_memory::Bus) -> u8 {
        let aud_1_len = yagber_memory::Aud1Len::from_bus(bus);
        let wave_duty = aud_1_len.wave_duty();
        wave_duty.at_step(self.duty_step_counter)
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

    fn get_initial_volume(&self, bus: &yagber_memory::Bus) -> u8 {
        let aud_1_env = yagber_memory::Audenv::ch1(bus);
        aud_1_env.initial_volume()
    }

    pub fn set_volume(&mut self, value: u8) {
        self.volume = value;
    }

    pub fn change_volume(&mut self, value: i8) {
        self.volume = self.volume.saturating_add_signed(value).min(15);
    }

    pub(crate) fn on_aud_1_high_write(apu: &mut Apu, bus: &mut yagber_memory::Bus, value: u8) {
        let aud_1_high = yagber_memory::Aud1High::new(value);
        if aud_1_high.trigger_enabled() {
            apu.ch1.trigger(bus);
        }
    }

    pub(crate) fn on_aud_1_env_write(apu: &mut Apu, value: u8) {
        let audenv = yagber_memory::Audenv::new(value);
        apu.ch1.set_volume(audenv.initial_volume());
    }
}
