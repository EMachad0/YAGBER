use std::fmt::Debug;

use crate::bus::Bus;

pub trait WriteObserver {
    /// Called after a write occurs to the memory.
    fn write(&mut self, ram: &mut Bus, address: u16, value: u8);
}

impl Debug for Box<dyn WriteObserver> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WriteObserver")
    }
}
