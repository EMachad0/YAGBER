use crate::Apu;

#[derive(Debug)]
pub struct WaveChannel {
    wave_index: u8,
    clock: u16,
    length_counter: u8,
    volume: yagber_memory::Aud3Volume,
    pub sample: u8,
}

impl WaveChannel {
    const DEFAULT_LENGTH_COUNTER: u8 = 64;

    pub fn new() -> Self {
        Self {
            wave_index: 0,
            clock: 0,
            length_counter: 0,
            volume: yagber_memory::Aud3Volume::Mute,
            sample: 0,
        }
    }

    pub fn trigger(&mut self, bus: &yagber_memory::Bus) {
        self.length_counter = self.get_initial_length_counter(bus);
        self.volume = self.get_initial_volume(bus);
        self.clock = super::PulseChannel::get_initial_period(bus, yagber_memory::AudioChannel::Ch3);
        self.wave_index = 0;
    }

    pub fn tick(&mut self, bus: &yagber_memory::Bus) {
        self.clock = self.clock.wrapping_sub(1);
        if self.clock != 0 {
            return;
        }
        self.clock = super::PulseChannel::get_initial_period(bus, yagber_memory::AudioChannel::Ch3);

        self.wave_index = (self.wave_index + 1) % 32;

        let wav_ram = yagber_memory::WavRam::from_bus(bus);
        let sample = wav_ram.read(self.wave_index);

        self.sample = sample >> self.volume.as_shift();
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

    fn get_initial_volume(&self, bus: &yagber_memory::Bus) -> yagber_memory::Aud3Volume {
        yagber_memory::Aud3Level::from_bus(bus).initial_volume()
    }

    pub(crate) fn on_aud_3_high_write(apu: &mut Apu, bus: &mut yagber_memory::Bus, value: u8) {
        let aud_3_high = yagber_memory::Aud3High::new(value);
        if aud_3_high.trigger_enabled() {
            apu.ch3.trigger(bus);
        }
    }
}

impl Default for WaveChannel {
    fn default() -> Self {
        Self::new()
    }
}
