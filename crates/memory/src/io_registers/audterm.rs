use crate::{Bus, IOType};

pub struct Audterm {
    value: u8,
}

impl Audterm {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            value: bus.read(IOType::AUDTERM.address()),
        }
    }

    pub fn ch4_left(&self) -> bool {
        self.value & 0x80 != 0
    }

    pub fn ch3_left(&self) -> bool {
        self.value & 0x40 != 0
    }

    pub fn ch2_left(&self) -> bool {
        self.value & 0x20 != 0
    }

    pub fn ch1_left(&self) -> bool {
        self.value & 0x10 != 0
    }

    pub fn ch4_right(&self) -> bool {
        self.value & 0x08 != 0
    }

    pub fn ch3_right(&self) -> bool {
        self.value & 0x04 != 0
    }

    pub fn ch2_right(&self) -> bool {
        self.value & 0x02 != 0
    }

    pub fn ch1_right(&self) -> bool {
        self.value & 0x01 != 0
    }
}
