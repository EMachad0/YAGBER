use crate::{Apu, channels::Envelope};

#[derive(Debug)]
pub struct NoiseChannel {
    lfsr: u16,
    clock: u16,
    length_counter: u8,
    volume: u8,
    pub envelope: Envelope,
    pub sample: u8,
}

impl NoiseChannel {
    const DEFAULT_LENGTH_COUNTER: u8 = 64;

    pub fn new() -> Self {
        Self {
            lfsr: 0,
            clock: 0,
            length_counter: 0,
            volume: 0,
            envelope: Envelope::new(),
            sample: 0,
        }
    }

    pub fn trigger(&mut self, bus: &yagber_memory::Bus) {
        self.length_counter = self.get_initial_length_counter(bus);
        self.volume = self.get_initial_volume(bus);
        self.lfsr = 0;

        let audenv = yagber_memory::Audenv::ch4(bus);
        self.envelope.set_timer(audenv.sweep_pace());
    }

    pub fn tick(&mut self, bus: &yagber_memory::Bus) {
        self.clock = self.clock.wrapping_sub(1);
        if self.clock != 0 {
            return;
        }
        let aud_4_poly = yagber_memory::Aud4Poly::from_bus(bus);
        self.clock = aud_4_poly.clock_divider() as u16 * (1 << aud_4_poly.clock_shift());

        let bit_value = (self.lfsr & 1) ^ ((self.lfsr & 2) >> 1);
        self.set_lfsr_bit(15, bit_value);

        if aud_4_poly.lfsr_mode() == yagber_memory::LfsrMode::Bit7 {
            self.set_lfsr_bit(7, bit_value);
        }

        self.lfsr >>= 1;
        let sampler_bit = (self.lfsr & 1) as u8;
        self.sample = sampler_bit * self.volume;
    }

    fn set_lfsr_bit(&mut self, bit: u8, value: u16) {
        if value == 1 {
            self.lfsr |= 1 << bit;
        } else {
            self.lfsr &= !(1 << bit);
        }
    }

    pub fn decrement_length_counter(&mut self) -> u8 {
        self.length_counter = self.length_counter.saturating_sub(1);
        self.length_counter
    }

    fn get_initial_length_counter(&self, bus: &yagber_memory::Bus) -> u8 {
        let current = self.length_counter;
        let initial = yagber_memory::Aud4Len::from_bus(bus).timer_length();
        if current != 0 {
            current
        } else if initial != 0 {
            initial
        } else {
            Self::DEFAULT_LENGTH_COUNTER
        }
    }

    fn get_initial_volume(&self, bus: &yagber_memory::Bus) -> u8 {
        yagber_memory::Audenv::ch4(bus).initial_volume()
    }

    pub fn set_volume(&mut self, value: u8) {
        self.volume = value;
    }

    pub fn change_volume(&mut self, value: i8) {
        self.volume = self.volume.saturating_add_signed(value).min(15);
    }

    pub(crate) fn on_aud_4_go_write(apu: &mut Apu, bus: &mut yagber_memory::Bus, value: u8) {
        let aud_4_go = yagber_memory::Aud4Go::new(value);
        if aud_4_go.trigger_enabled() {
            apu.ch4.trigger(bus);
        }
    }

    pub(crate) fn on_aud_4_env_write(apu: &mut Apu, value: u8) {
        let audenv = yagber_memory::Audenv::new(value);
        apu.ch4.set_volume(audenv.initial_volume());
    }
}

impl Default for NoiseChannel {
    fn default() -> Self {
        Self::new()
    }
}
