use crate::{Bus, IOType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LfsrMode {
    Bit15,
    Bit7,
}

#[derive(Debug, Clone, Copy)]
pub struct Aud4Poly {
    value: u8,
}

impl Aud4Poly {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self::new(bus.read(IOType::AUD4POLY.address()))
    }

    pub fn clock_shift(&self) -> u8 {
        (self.value & 0xF0) >> 4
    }

    pub fn lfsr_mode(&self) -> LfsrMode {
        if self.value & 0x08 == 0 {
            LfsrMode::Bit15
        } else {
            LfsrMode::Bit7
        }
    }

    pub fn clock_divider(&self) -> u8 {
        let value = self.value & 0x07;
        if value == 0 { 8 } else { 16 * value }
    }
}
