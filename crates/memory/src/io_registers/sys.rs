use crate::{Bus, IOType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysMode {
    Cgb,
    Dmg,
}

#[derive(Debug, Clone, Copy)]
pub struct SysRegister {
    value: u8,
}

impl SysRegister {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self::new(bus.read(IOType::SYS.address()))
    }

    pub fn mode(&self) -> SysMode {
        if self.value & 0x04 == 0 {
            SysMode::Cgb
        } else {
            SysMode::Dmg
        }
    }
}
