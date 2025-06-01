use yagber_ram::{MemoryObserver, Ram};

use crate::timer::DIV_ADDR;

#[derive(Debug, Default)]
pub struct RamObserver;

impl RamObserver {
    pub fn new() -> Self {
        Self
    }
}

impl MemoryObserver for RamObserver {
    fn on_write(&mut self, ram: &mut Ram, address: u16, value: u8) {
        if address == DIV_ADDR && value != 0 {
            ram.write(address, 0);
        }
    }
}
