use crate::{Bus, io_registers::IOType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioChannel {
    Ch1 = 0,
    Ch2 = 1,
    Ch3 = 2,
    Ch4 = 3,
}

impl AudioChannel {
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Self::Ch1,
            1 => Self::Ch2,
            2 => Self::Ch3,
            3 => Self::Ch4,
            _ => panic!("Invalid audio channel index: {index}"),
        }
    }

    pub fn index(&self) -> u8 {
        *self as u8
    }

    pub fn audena_bit(&self) -> u8 {
        1 << self.index()
    }
}

pub struct Audena {
    value: u8,
}

impl Audena {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            value: bus.read(IOType::AUDENA.address()),
        }
    }

    pub fn apu_enabled(&self) -> bool {
        self.value & 0x80 != 0
    }

    pub fn ch_enabled(&self, channel: AudioChannel) -> bool {
        self.value & (1 << channel as u8) != 0
    }

    fn on_aud_x_env_write(bus: &mut Bus, value: u8, channel: AudioChannel) {
        let audenv = super::Audenv::new(value);
        if audenv.dac_enabled() {
            return;
        }

        let audena = bus.read(IOType::AUDENA.address());
        let new_audena = audena & !channel.audena_bit();
        bus.io_registers
            .write_unchecked(IOType::AUDENA.address(), new_audena);
    }

    pub(crate) fn on_aud_1_env_write(bus: &mut Bus, value: u8) {
        Self::on_aud_x_env_write(bus, value, AudioChannel::Ch1);
    }

    pub(crate) fn on_aud_2_env_write(bus: &mut Bus, value: u8) {
        Self::on_aud_x_env_write(bus, value, AudioChannel::Ch2);
    }

    pub(crate) fn on_aud_4_env_write(bus: &mut Bus, value: u8) {
        Self::on_aud_x_env_write(bus, value, AudioChannel::Ch4);
    }

    pub(crate) fn on_aud_3_ena_write(bus: &mut Bus, value: u8) {
        let aud3ena = super::Aud3Ena::new(value);
        if aud3ena.dac_enabled() {
            return;
        }

        let audena = bus.read(IOType::AUDENA.address());
        let new_audena = audena & !AudioChannel::Ch3.audena_bit();
        bus.io_registers
            .write_unchecked(IOType::AUDENA.address(), new_audena);
    }

    pub(crate) fn on_aud_1_high_write(bus: &mut Bus, value: u8) {
        let aud1high = super::Aud1High::new(value);
        if aud1high.trigger_enabled() {
            let audena = bus.read(IOType::AUDENA.address());
            let new_audena = audena | AudioChannel::Ch1.audena_bit();
            bus.io_registers
                .write_unchecked(IOType::AUDENA.address(), new_audena);
        }
    }

    pub(crate) fn on_aud_2_high_write(bus: &mut Bus, value: u8) {
        let aud2high = super::Aud2High::new(value);
        if aud2high.trigger_enabled() {
            let audena = bus.read(IOType::AUDENA.address());
            let new_audena = audena | AudioChannel::Ch2.audena_bit();
            bus.io_registers
                .write_unchecked(IOType::AUDENA.address(), new_audena);
        }
    }

    pub(crate) fn on_aud_3_high_write(bus: &mut Bus, value: u8) {
        let aud3high = super::Aud3High::new(value);
        if aud3high.trigger_enabled() {
            let audena = bus.read(IOType::AUDENA.address());
            let new_audena = audena | AudioChannel::Ch3.audena_bit();
            bus.io_registers
                .write_unchecked(IOType::AUDENA.address(), new_audena);
        }
    }

    pub(crate) fn on_aud_4_go_write(bus: &mut Bus, value: u8) {
        let aud4go = super::Aud4Go::new(value);
        if aud4go.trigger_enabled() {
            let audena = bus.read(IOType::AUDENA.address());
            let new_audena = audena | AudioChannel::Ch4.audena_bit();
            bus.io_registers
                .write_unchecked(IOType::AUDENA.address(), new_audena);
        }
    }

    pub(crate) fn audena_transformer((old_value, new_value): (u8, u8)) -> Option<u8> {
        Some((new_value & 0x80) | (old_value & 0x0F))
    }
}
