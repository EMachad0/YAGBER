use std::fmt::Debug;

use crate::bus::Bus;

pub trait MemoryObserver {
    /// Called when the observer is added to the bus.
    fn on_add(&mut self, _ram: &mut Bus) {}
    /// Called after a write occurs to the memory.
    fn on_write(&mut self, ram: &mut Bus, address: u16, value: u8);
}

impl Debug for Box<dyn MemoryObserver> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WriteObserver")
    }
}
