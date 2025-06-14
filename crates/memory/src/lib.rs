mod boot_rom;
mod bus;
mod cartridge;
mod events;
mod interrupt;
mod io_registers;
mod mbc;
mod memory;
mod memory_wrapper;
mod oam;
mod ram;
mod register;
mod vram;

pub use bus::Bus;
pub use events::MemoryWriteEvent;
pub use interrupt::InterruptType;
pub use memory::Memory;
pub use memory_wrapper::MemoryWrapper;

#[macro_use]
extern crate tracing;

pub struct MemoryPlugin {
    memory_bus: Option<Bus>,
}

impl MemoryPlugin {
    pub fn new() -> Self {
        Self {
            memory_bus: Some(Bus::new()),
        }
    }

    pub fn with_cartridge(mut self, data: &[u8]) -> Self {
        self.memory_bus.as_mut().unwrap().load_rom(data);
        self
    }
}

impl Default for MemoryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl yagber_app::Plugin for MemoryPlugin {
    fn init(mut self, emulator: &mut yagber_app::Emulator) {
        let memory_bus = std::mem::take(&mut self.memory_bus).unwrap();
        emulator
            .with_component(memory_bus)
            .with_event::<MemoryWriteEvent>();
    }
}
