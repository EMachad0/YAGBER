use yagber_memory::AudioChannel;

use crate::{Apu, channels::Envelope};

#[derive(Debug)]
pub struct PulseChannel {
    clock: u16,
    duty_step_counter: u8,
    length_counter: u8,
    volume: u8,
    pub envelope: Envelope,
    pub sample: u8,
    channel: AudioChannel,
}

impl PulseChannel {
    const DEFAULT_LENGTH_COUNTER: u8 = 64;

    pub fn new(channel: AudioChannel) -> Self {
        Self {
            clock: 0,
            length_counter: 0,
            duty_step_counter: 0,
            volume: 0,
            envelope: Envelope::new(),
            sample: 0,
            channel,
        }
    }

    pub fn trigger(&mut self, bus: &yagber_memory::Bus) {
        self.length_counter = self.get_initial_length_counter(bus);
        self.clock = Self::get_initial_period(bus, self.channel);
        self.volume = self.get_initial_volume(bus);
        self.duty_step_counter = 0;

        let audenv = match self.channel {
            AudioChannel::Ch1 => yagber_memory::Audenv::ch1(bus),
            AudioChannel::Ch2 => yagber_memory::Audenv::ch2(bus),
            _ => unreachable!(),
        };
        self.envelope.set_timer(audenv.sweep_pace());
    }

    pub fn tick(&mut self, bus: &yagber_memory::Bus) {
        self.clock = self.clock.wrapping_sub(1);
        if self.clock != 0 {
            return;
        }
        self.clock = Self::get_initial_period(bus, self.channel);

        let sampler_bit = self.duty_step(bus);

        self.duty_step_counter = (self.duty_step_counter + 1) % 8;

        self.sample = sampler_bit * self.volume;
    }

    fn duty_step(&self, bus: &yagber_memory::Bus) -> u8 {
        let wave_duty = match self.channel {
            AudioChannel::Ch1 => yagber_memory::Aud1Len::from_bus(bus).wave_duty(),
            AudioChannel::Ch2 => yagber_memory::Aud2Len::from_bus(bus).wave_duty(),
            _ => unreachable!(),
        };
        wave_duty.at_step(self.duty_step_counter)
    }

    pub fn decrement_length_counter(&mut self) -> u8 {
        self.length_counter = self.length_counter.saturating_sub(1);
        self.length_counter
    }

    pub(crate) fn get_initial_period(bus: &yagber_memory::Bus, channel: AudioChannel) -> u16 {
        let high = match channel {
            AudioChannel::Ch1 => yagber_memory::Aud1High::from_bus(bus).period_high() as u16,
            AudioChannel::Ch2 => yagber_memory::Aud2High::from_bus(bus).period_high() as u16,
            AudioChannel::Ch3 => yagber_memory::Aud3High::from_bus(bus).period_high() as u16,
            _ => unreachable!(),
        };
        let low_io_type = match channel {
            AudioChannel::Ch1 => yagber_memory::IOType::AUD1LOW,
            AudioChannel::Ch2 => yagber_memory::IOType::AUD2LOW,
            AudioChannel::Ch3 => yagber_memory::IOType::AUD3LOW,
            _ => unreachable!(),
        };
        let low = bus.read(low_io_type.address()) as u16;
        let period = (high << 8) | low;
        0x800 - period
    }

    fn get_initial_length_counter(&self, bus: &yagber_memory::Bus) -> u8 {
        let current = self.length_counter;
        let initial = match self.channel {
            AudioChannel::Ch1 => yagber_memory::Aud1Len::from_bus(bus).timer_length(),
            AudioChannel::Ch2 => yagber_memory::Aud2Len::from_bus(bus).timer_length(),
            _ => unreachable!(),
        };
        if current != 0 {
            current
        } else if initial != 0 {
            initial
        } else {
            Self::DEFAULT_LENGTH_COUNTER
        }
    }

    fn get_initial_volume(&self, bus: &yagber_memory::Bus) -> u8 {
        let aud_env = match self.channel {
            AudioChannel::Ch1 => yagber_memory::Audenv::ch1(bus),
            AudioChannel::Ch2 => yagber_memory::Audenv::ch2(bus),
            _ => unreachable!(),
        };
        aud_env.initial_volume()
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

    pub(crate) fn on_aud_2_high_write(apu: &mut Apu, bus: &mut yagber_memory::Bus, value: u8) {
        let aud_2_high = yagber_memory::Aud2High::new(value);
        if aud_2_high.trigger_enabled() {
            apu.ch2.trigger(bus);
        }
    }

    pub(crate) fn on_aud_2_env_write(apu: &mut Apu, value: u8) {
        let audenv = yagber_memory::Audenv::new(value);
        apu.ch2.set_volume(audenv.initial_volume());
    }
}
