use yagber_memory::{Bus, MemoryObserver};

use crate::timer::DIV_ADDR;

#[derive(Debug, Default)]
pub struct DivObserver;

impl DivObserver {
    pub fn new() -> Self {
        Self
    }
}

impl MemoryObserver for DivObserver {
    fn on_write(&mut self, bus: &mut Bus, address: u16, value: u8) {
        if address == DIV_ADDR && value != 0 {
            bus.write(address, 0);
        }
    }
}
