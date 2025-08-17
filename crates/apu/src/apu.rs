use crate::{
    AudioBuffer,
    channels::{NoiseChannel, PulseChannel, WaveChannel},
    high_pass_filter::HighPassFilter,
    sweep::Sweep,
};

#[derive(Debug)]
pub struct Apu {
    channel_step_accumulator: u8,
    sampler_accumulator: u32,
    sample_rate_hz: u32,
    tcycles_per_sample: u32,
    pub ch1: PulseChannel,
    pub ch2: PulseChannel,
    pub ch3: WaveChannel,
    pub ch4: NoiseChannel,
    pub left_buffer: AudioBuffer,
    pub right_buffer: AudioBuffer,
    high_pass_filter: HighPassFilter,
    pub sweep: Sweep,
}

impl Apu {
    const DEFAULT_SAMPLE_RATE_HZ: u32 = 48_000;

    pub fn new() -> Self {
        let sample_rate_hz = Self::DEFAULT_SAMPLE_RATE_HZ;
        let tcycles_per_sample = yagber_app::Emulator::TARGET_DOT_FREQ_HZ / sample_rate_hz;

        Self {
            channel_step_accumulator: 0,
            sampler_accumulator: 0,
            sample_rate_hz,
            tcycles_per_sample,
            ch1: PulseChannel::new(yagber_memory::AudioChannel::Ch1),
            ch2: PulseChannel::new(yagber_memory::AudioChannel::Ch2),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            left_buffer: AudioBuffer::new_with_seconds_at_rate(sample_rate_hz, 3),
            right_buffer: AudioBuffer::new_with_seconds_at_rate(sample_rate_hz, 3),
            high_pass_filter: HighPassFilter::new(sample_rate_hz),
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

        self.channel_step_accumulator += 1;
        if self.channel_step_accumulator >= 4 {
            self.channel_step_accumulator -= 4;
            self.channel_tick(bus, yagber_memory::AudioChannel::Ch1);
            self.channel_tick(bus, yagber_memory::AudioChannel::Ch2);
            self.channel_tick(bus, yagber_memory::AudioChannel::Ch3);
            self.channel_tick(bus, yagber_memory::AudioChannel::Ch4);
        }

        self.sampler_accumulator += 1;
        while self.sampler_accumulator >= self.tcycles_per_sample {
            self.sampler_accumulator -= self.tcycles_per_sample;

            let ch1 = self.get_channel_sample(bus, yagber_memory::AudioChannel::Ch1);
            let ch2 = self.get_channel_sample(bus, yagber_memory::AudioChannel::Ch2);
            let ch3 = self.get_channel_sample(bus, yagber_memory::AudioChannel::Ch3);
            let ch4 = self.get_channel_sample(bus, yagber_memory::AudioChannel::Ch4);

            let audterm = yagber_memory::Audterm::from_bus(bus);
            let (mut left_sample, mut right_sample) = self.mixer(audterm, ch1, ch2, ch3, ch4);

            let audvol = yagber_memory::Audvol::from_bus(bus);
            left_sample *= audvol.left_volume();
            right_sample *= audvol.right_volume();

            let (left_sample, right_sample) =
                self.high_pass_filter.apply(left_sample, right_sample);

            self.left_buffer.push(left_sample);
            self.right_buffer.push(right_sample);
        }
    }

    fn channel_tick(&mut self, bus: &mut yagber_memory::Bus, channel: yagber_memory::AudioChannel) {
        let audena = yagber_memory::Audena::from_bus(bus);
        let ch_enabled = audena.ch_enabled(channel);
        if ch_enabled {
            match channel {
                yagber_memory::AudioChannel::Ch1 => self.ch1.tick(bus),
                yagber_memory::AudioChannel::Ch2 => self.ch2.tick(bus),
                yagber_memory::AudioChannel::Ch3 => self.ch3.tick(bus),
                yagber_memory::AudioChannel::Ch4 => self.ch4.tick(bus),
            }
        }
    }

    fn mixer(
        &self,
        audterm: yagber_memory::Audterm,
        ch1_sample: f32,
        ch2_sample: f32,
        ch3_sample: f32,
        ch4_sample: f32,
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
        if audterm.ch3_left() {
            left += ch3_sample;
            left_count += 1;
        }
        if audterm.ch3_right() {
            right += ch3_sample;
            right_count += 1;
        }
        if audterm.ch4_left() {
            left += ch4_sample;
            left_count += 1;
        }
        if audterm.ch4_right() {
            right += ch4_sample;
            right_count += 1;
        }

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

    fn get_channel_sample(
        &self,
        bus: &mut yagber_memory::Bus,
        channel: yagber_memory::AudioChannel,
    ) -> f32 {
        let raw_sample = match channel {
            yagber_memory::AudioChannel::Ch1 => self.ch1.sample,
            yagber_memory::AudioChannel::Ch2 => self.ch2.sample,
            yagber_memory::AudioChannel::Ch3 => self.ch3.sample,
            yagber_memory::AudioChannel::Ch4 => self.ch4.sample,
        };
        let dac_enabled = match channel {
            yagber_memory::AudioChannel::Ch1 => yagber_memory::Audenv::ch1(bus).dac_enabled(),
            yagber_memory::AudioChannel::Ch2 => yagber_memory::Audenv::ch2(bus).dac_enabled(),
            yagber_memory::AudioChannel::Ch3 => yagber_memory::Aud3Ena::from_bus(bus).dac_enabled(),
            yagber_memory::AudioChannel::Ch4 => yagber_memory::Audenv::ch4(bus).dac_enabled(),
        };
        if dac_enabled {
            self.dac_transform(raw_sample)
        } else {
            0.0
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate_hz: u32) {
        #[cfg(feature = "trace")]
        tracing::trace!("Setting APU sample rate to: {sample_rate_hz} Hz");
        self.sample_rate_hz = sample_rate_hz.max(1);
        let cycles_per_second = yagber_app::Emulator::TARGET_DOT_FREQ_HZ;
        self.tcycles_per_sample = cycles_per_second / self.sample_rate_hz;
        self.high_pass_filter.set_sample_rate(self.sample_rate_hz);

        // Resize buffers to ~3 seconds at the new rate if the audio backend
        // hasn't taken ownership of the consumers yet. This avoids breaking
        // an already running stream.
        if self.left_buffer.consumer.is_some() {
            self.left_buffer = AudioBuffer::new_with_seconds_at_rate(self.sample_rate_hz, 3);
        }
        if self.right_buffer.consumer.is_some() {
            self.right_buffer = AudioBuffer::new_with_seconds_at_rate(self.sample_rate_hz, 3);
        }
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

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch3) {
            let aud_3_high = yagber_memory::Aud3High::from_bus(bus);
            if aud_3_high.length_enabled() {
                let length_counter = self.ch3.decrement_length_counter();
                if length_counter == 0 {
                    new_audena &= !yagber_memory::AudioChannel::Ch3.audena_bit();
                }
            }
        }

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch4) {
            let aud_4_go = yagber_memory::Aud4Go::from_bus(bus);
            if aud_4_go.length_enabled() {
                let length_counter = self.ch4.decrement_length_counter();
                if length_counter == 0 {
                    new_audena &= !yagber_memory::AudioChannel::Ch4.audena_bit();
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

        if audena.ch_enabled(yagber_memory::AudioChannel::Ch4) {
            let audenv = yagber_memory::Audenv::ch4(bus);
            let envelope_change = self.ch4.envelope.tick(&audenv);
            self.ch4.change_volume(envelope_change);
        }
    }
}

impl yagber_app::Component for Apu {}

impl Default for Apu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use ringbuf::traits::Consumer;

    use super::*;

    #[test]
    fn test_dac_transform() {
        let mut bus = yagber_memory::Bus::new();
        bus.io_registers
            .write(yagber_memory::IOType::AUDENA.address(), 0xFF);

        let mut apu = Apu::new();
        apu.set_sample_rate(1_048_576);

        for _ in 0..70224 {
            apu.tick(&mut bus);
        }

        let mut left_consumer = apu.left_buffer.take_consumer().unwrap();
        let mut right_consumer = apu.right_buffer.take_consumer().unwrap();
        assert_eq!(left_consumer.pop_iter().count(), 70224 / 4);
        assert_eq!(right_consumer.pop_iter().count(), 70224 / 4);
    }
}
