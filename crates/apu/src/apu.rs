use crate::{AudioBuffer, channels::PulseChannel, high_pass_filter::HighPassFilter, sweep::Sweep};

#[derive(Debug)]
pub struct Apu {
    cycles: u8,
    pub ch1: PulseChannel,
    pub ch2: PulseChannel,
    pub left_buffer: AudioBuffer,
    pub right_buffer: AudioBuffer,
    high_pass_filter: HighPassFilter,
    pub sweep: Sweep,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            ch1: PulseChannel::new(yagber_memory::AudioChannel::Ch1),
            ch2: PulseChannel::new(yagber_memory::AudioChannel::Ch2),
            left_buffer: AudioBuffer::new(),
            right_buffer: AudioBuffer::new(),
            high_pass_filter: HighPassFilter::new(),
            sweep: Sweep::new(),
        }
    }

    pub(crate) fn on_tcycle(emulator: &mut yagber_app::Emulator) {
        let (apu, bus) = emulator
            .get_components_mut2::<Self, yagber_memory::Bus>()
            .expect("Apu and/or Bus components not found");

        apu.tick(bus);
    }

    /// Ticks the APU.
    /// Meant to be called every T-cycle.
    fn tick(&mut self, bus: &mut yagber_memory::Bus) {
        let audena = yagber_memory::Audena::from_bus(bus);
        if !audena.apu_enabled() {
            return;
        }

        self.cycles += 1;
        if self.cycles == 4 {
            self.cycles = 0;
        }
        if self.cycles != 0 {
            return;
        }

        let ch1_sample = self.channel_sample(bus, yagber_memory::AudioChannel::Ch1);
        let ch2_sample = self.channel_sample(bus, yagber_memory::AudioChannel::Ch2);
        let ch3_sample = self.channel_sample(bus, yagber_memory::AudioChannel::Ch3);
        let ch4_sample = self.channel_sample(bus, yagber_memory::AudioChannel::Ch4);

        let audterm = yagber_memory::Audterm::from_bus(bus);
        let (left_sample, right_sample) =
            self.mixer(audterm, ch1_sample, ch2_sample, ch3_sample, ch4_sample);

        let audvol = yagber_memory::Audvol::from_bus(bus);
        let left_sample = left_sample * audvol.left_volume();
        let right_sample = right_sample * audvol.right_volume();

        let (left_sample, right_sample) = self.high_pass_filter.apply(left_sample, right_sample);

        let mut left_lock = self.left_buffer.lock();
        let mut right_lock = self.right_buffer.lock();
        left_lock.push(left_sample);
        right_lock.push(right_sample);
    }

    fn channel_sample(
        &mut self,
        bus: &mut yagber_memory::Bus,
        channel: yagber_memory::AudioChannel,
    ) -> f32 {
        let audena = yagber_memory::Audena::from_bus(bus);
        let ch_enabled = audena.ch_enabled(channel);
        if ch_enabled {
            match channel {
                yagber_memory::AudioChannel::Ch1 => self.ch1.tick(bus),
                yagber_memory::AudioChannel::Ch2 => self.ch2.tick(bus),
                yagber_memory::AudioChannel::Ch3 => {}
                yagber_memory::AudioChannel::Ch4 => {}
            }
        }
        let sample = match channel {
            yagber_memory::AudioChannel::Ch1 => self.ch1.sample,
            yagber_memory::AudioChannel::Ch2 => self.ch2.sample,
            yagber_memory::AudioChannel::Ch3 => 7,
            yagber_memory::AudioChannel::Ch4 => 7,
        };
        let dac_enabled = match channel {
            yagber_memory::AudioChannel::Ch1 => yagber_memory::Audenv::ch1(bus).dac_enabled(),
            yagber_memory::AudioChannel::Ch2 => yagber_memory::Audenv::ch2(bus).dac_enabled(),
            yagber_memory::AudioChannel::Ch3 => yagber_memory::Aud3Ena::from_bus(bus).dac_enabled(),
            yagber_memory::AudioChannel::Ch4 => yagber_memory::Audenv::ch4(bus).dac_enabled(),
        };
        if !dac_enabled {
            0.0
        } else {
            self.dac_transform(sample)
        }
    }

    fn mixer(
        &self,
        audterm: yagber_memory::Audterm,
        ch1_sample: f32,
        ch2_sample: f32,
        _ch3_sample: f32,
        _ch4_sample: f32,
    ) -> (f32, f32) {
        let mut left = 0.0;
        let mut right = 0.0;
        let mut left_count = 0;
        let mut right_count = 0;

        if audterm.ch1_left() {
            left += ch1_sample;
            left_count += 1;
        }
        if audterm.ch1_right() {
            right += ch1_sample;
            right_count += 1;
        }
        if audterm.ch2_left() {
            left += ch2_sample;
            left_count += 1;
        }
        if audterm.ch2_right() {
            right += ch2_sample;
            right_count += 1;
        }
        // if audterm.ch3_left() {
        //     left += ch3_sample;
        //     left_count += 1;
        // }
        // if audterm.ch3_right() {
        //     right += ch3_sample;
        //     right_count += 1;
        // }
        // if audterm.ch4_left() {
        //     left += ch4_sample;
        //     left_count += 1;
        // }
        // if audterm.ch4_right() {
        //     right += ch4_sample;
        //     right_count += 1;
        // }

        let left_count = left_count.max(1) as f32;
        let right_count = right_count.max(1) as f32;
        let left = left / left_count;
        let right = right / right_count;

        (left, right)
    }

    fn dac_transform(&self, sample: u8) -> f32 {
        // 0..15 -> 1..-1
        let sample = sample as f32;
        let sample = sample / 15.0;
        let sample = 1.0 - sample;
        sample * 2.0 - 1.0
    }

    pub(crate) fn tick_sound_length(&mut self, bus: &mut yagber_memory::Bus) {
        let audena_value = bus
            .io_registers
            .read(yagber_memory::IOType::AUDENA.address());
        let audena = yagber_memory::Audena::new(audena_value);
        let mut new_audena = audena_value;

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch1) {
            let aud_1_high = yagber_memory::Aud1High::from_bus(bus);
            if aud_1_high.length_enabled() {
                let length_counter = self.ch1.decrement_length_counter();
                if length_counter == 0 {
                    new_audena &= !yagber_memory::AudioChannel::Ch1.audena_bit();
                }
            }
        }

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch2) {
            let aud_2_high = yagber_memory::Aud2High::from_bus(bus);
            if aud_2_high.length_enabled() {
                let length_counter = self.ch2.decrement_length_counter();
                if length_counter == 0 {
                    new_audena &= !yagber_memory::AudioChannel::Ch2.audena_bit();
                }
            }
        }

        bus.io_registers
            .write(yagber_memory::IOType::AUDENA.address(), new_audena);
    }

    pub(crate) fn tick_envelope(&mut self, bus: &mut yagber_memory::Bus) {
        let audena = yagber_memory::Audena::from_bus(bus);

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch1) {
            let audenv = yagber_memory::Audenv::ch1(bus);
            let envelope_change = self.ch1.envelope.tick(&audenv);
            self.ch1.change_volume(envelope_change);
        }

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch2) {
            let audenv = yagber_memory::Audenv::ch2(bus);
            let envelope_change = self.ch2.envelope.tick(&audenv);
            self.ch2.change_volume(envelope_change);
        }
    }
}

impl yagber_app::Component for Apu {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dac_transform() {
        let mut bus = yagber_memory::Bus::new();
        bus.io_registers
            .write(yagber_memory::IOType::AUDENA.address(), 0xFF);

        let mut apu = Apu::new();

        for _ in 0..70224 {
            apu.tick(&mut bus);
        }

        assert_eq!(apu.left_buffer.lock().len(), 70224 / 4);
        assert_eq!(apu.right_buffer.lock().len(), 70224 / 4);
    }
}
