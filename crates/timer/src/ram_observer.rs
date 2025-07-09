use yagber_memory::Bus;

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
        if event.address == yagber_memory::IOType::DIV.address() && event.value != 0 {
            let bus = emulator.get_component_mut::<Bus>().unwrap();
            bus.write(yagber_memory::IOType::DIV.address(), 0);
        }
    }
}
