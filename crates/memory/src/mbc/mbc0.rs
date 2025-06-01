use crate::mbc::Mbc;

pub struct Mbc0;

impl Mbc0 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Mbc for Mbc0 {
    fn rom_write(&mut self, _address: u16, _value: u8) {
        // No-op, MBC0 does not support writing
    }

    fn rom_address(&self, address: u16) -> usize {
        address as usize
    }

    fn ram_address(&self, address: u16) -> usize {
        address as usize
    }

    fn ram_enabled(&self) -> bool {
        true
    }
}
