use crate::{Bus, IOType};

pub enum SweepDirection {
    Increase,
    Decrease,
}

#[derive(Debug, Default)]
pub struct Aud1Sweep {
    value: u8,
}

impl Aud1Sweep {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            value: bus.read(IOType::AUD1SWEEP.address()),
        }
    }

    pub fn pace(&self) -> u8 {
        (self.value & 0x70) >> 4
    }

    pub fn direction(&self) -> SweepDirection {
        if self.value & 0x08 == 0 {
            SweepDirection::Increase
        } else {
            SweepDirection::Decrease
        }
    }

    pub fn step(&self) -> u8 {
        self.value & 0x07
    }

    pub fn enabled(&self) -> bool {
        self.pace() != 0
    }
}
