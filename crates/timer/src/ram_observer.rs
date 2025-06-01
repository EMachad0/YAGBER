use yagber_ram::{Ram, WriteObserver};

use crate::timer::DIV_ADDR;

pub struct RamObserver;

impl WriteObserver for RamObserver {
    fn write(&mut self, ram: &mut Ram, address: u16, value: u8) {
        if address == DIV_ADDR && value != 0 {
            ram.write(address, 0);
        }
    }
}
