use crate::{Bus, IOType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpriMode {
    Cgb,
    Dmg,
}

#[derive(Debug, Clone, Copy)]
pub struct OpriRegister {
    value: u8,
}

impl OpriRegister {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self::new(bus.read(IOType::OPRI.address()))
    }

    pub fn mode(&self) -> OpriMode {
        if self.value & 0x01 != 0 {
            OpriMode::Dmg
        } else {
            OpriMode::Cgb
        }
    }
}
