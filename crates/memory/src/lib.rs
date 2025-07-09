mod boot_rom;
mod bus;
mod cartridge;
mod cram;
mod events;
mod interrupt;
mod io_registers;
mod mbc;
mod memory;
mod oam;
mod ram;
mod register;
mod vram;
mod wram;

pub use bus::Bus;
pub use events::MemoryWriteEvent;
pub use interrupt::InterruptType;
pub use io_registers::{
    CramReaderRegister, CramWriterRegister, IOBus, IOType, LcdcRegister, StatRegister,
};
pub use memory::Memory;
pub use register::{ByteRegister, Register};

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
        let event_sender = emulator.event_sender();

        let mut memory_bus = std::mem::take(&mut self.memory_bus).unwrap();
        memory_bus.with_event_sender(event_sender);

        emulator
            .with_component(memory_bus)
            .with_event::<MemoryWriteEvent>()
            .with_event_handler(crate::vram::Vram::on_memory_write)
            .with_event_handler(crate::wram::Wram::on_memory_write)
            .with_event_handler(crate::io_registers::StatRegister::on_dot_cycle);
    }
}
