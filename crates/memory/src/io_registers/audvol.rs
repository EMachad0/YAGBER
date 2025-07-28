use crate::{Bus, IOType};

pub struct Audvol {
    value: u8,
}

impl Audvol {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            value: bus.read(IOType::AUDVOL.address()),
        }
    }

    pub fn vin_left(&self) -> bool {
        self.value & 0x80 != 0
    }

    pub fn vin_right(&self) -> bool {
        self.value & 0x08 != 0
    }

    pub fn left_volume(&self) -> u8 {
        ((self.value & 0x30) >> 4) + 1
    }

    pub fn right_volume(&self) -> u8 {
        (self.value & 0x03) + 1
    }
}
