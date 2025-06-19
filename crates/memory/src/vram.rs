use crate::{Bus, Memory, MemoryWriteEvent, ram::Ram};

#[derive(Debug)]
pub struct Vram {
    // gbc vram has two banks of 8kb each
    ram: [Ram; 2],
    current_bank: usize,
    accessible: bool,
}

impl Vram {
    const SIZE: usize = 0x2000;
    const OFFSET: usize = 0x8000;
    const BANK_SELECT_ADDRESS: u16 = 0xFF4F;

    pub fn new() -> Self {
        Self {
            ram: [
                Ram::new(Self::SIZE, Self::OFFSET),
                Ram::new(Self::SIZE, Self::OFFSET),
            ],
            current_bank: 0,
            accessible: true,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if self.accessible {
            self.ram[self.current_bank].read(address)
        } else {
            0xFF
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if self.accessible {
            self.ram[self.current_bank].write(address, value);
        }
    }

    pub(crate) fn set_accessible(&mut self, accessible: bool) {
        self.accessible = accessible;
    }

    pub fn on_memory_write(emulator: &mut yagber_app::Emulator, event: &MemoryWriteEvent) {
        if event.address == Self::BANK_SELECT_ADDRESS {
            let bank = event.value & 0x01;
            let memory_bus = emulator
                .get_component_mut::<Bus>()
                .expect("MemoryBus not found");
            memory_bus.vram_mut().set_bank(bank as usize);
        }
    }

    pub fn set_bank(&mut self, bank: usize) {
        self.current_bank = bank;
    }
}

impl Memory for Vram {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}

impl Default for Vram {
    fn default() -> Self {
        Self::new()
    }
}
