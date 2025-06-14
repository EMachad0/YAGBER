use yagber_memory::Bus;

use crate::timer::DIV_ADDR;

#[derive(Debug, Default)]
pub struct DivObserver;

impl DivObserver {
    pub fn new() -> Self {
        Self
    }

    pub fn on_memory_write(
        emulator: &mut yagber_app::Emulator,
        event: &yagber_memory::MemoryWriteEvent,
    ) {
        if event.address == DIV_ADDR && event.value != 0 {
            let bus = emulator.get_component_mut::<Bus>().unwrap();
            bus.write(DIV_ADDR, 0);
        }
    }
}
