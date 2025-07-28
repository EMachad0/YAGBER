use crate::{Bus, IOType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeDirection {
    Increase,
    Decrease,
}

#[derive(Debug, Clone, Copy)]
pub struct Audenv {
    value: u8,
}

impl Audenv {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus, io_type: IOType) -> Self {
        if !matches!(io_type, IOType::AUD1ENV | IOType::AUD2ENV | IOType::AUD4ENV) {
            panic!("Invalid IO type for Audenv: {:?}", io_type);
        }

        Self {
            value: bus.read(io_type.address()),
        }
    }

    pub fn ch1(bus: &Bus) -> Self {
        Self::from_bus(bus, IOType::AUD1ENV)
    }

    pub fn ch2(bus: &Bus) -> Self {
        Self::from_bus(bus, IOType::AUD2ENV)
    }

    pub fn ch4(bus: &Bus) -> Self {
        Self::from_bus(bus, IOType::AUD4ENV)
    }

    pub fn initial_volume(&self) -> u8 {
        (self.value & 0xF0) >> 4
    }

    pub fn direction(&self) -> EnvelopeDirection {
        if self.value & 0x08 == 0 {
            EnvelopeDirection::Decrease
        } else {
            EnvelopeDirection::Increase
        }
    }

    pub fn sweep_pace(&self) -> u8 {
        self.value & 0x07
    }

    pub fn dac_enabled(&self) -> bool {
        self.value & 0xF8 != 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud3Ena {
    value: u8,
}

impl Aud3Ena {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            value: bus.read(IOType::AUD3ENA.address()),
        }
    }

    pub fn dac_enabled(&self) -> bool {
        self.value & 0x80 != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aud3Volume {
    Mute,
    Quarter,
    Half,
    Full,
}

impl Aud3Volume {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0b00 => Self::Mute,
            0b01 => Self::Quarter,
            0b10 => Self::Half,
            0b11 => Self::Full,
            _ => panic!("Invalid volume value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud3Level {
    value: u8,
}

impl Aud3Level {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            value: bus.read(IOType::AUD3LEVEL.address()),
        }
    }

    pub fn initial_volume(&self) -> Aud3Volume {
        let value = ((self.value & 0xF0) >> 4) & 0b11;
        Aud3Volume::from_u8(value)
    }
}
