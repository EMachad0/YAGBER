use yagber_app::Emulator;

use crate::{Bus, MemoryWriteEvent};

pub trait MemoryWrapper {
    fn write(&mut self, address: u16, value: u8);
}

impl MemoryWrapper for Emulator {
    fn write(&mut self, address: u16, value: u8) {
        let memory_bus = self.get_component_mut::<Bus>().unwrap();
        memory_bus.write(address, value);
        self.emit_event(MemoryWriteEvent { address, value });
    }
}
