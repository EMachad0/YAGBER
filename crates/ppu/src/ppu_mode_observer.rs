use yagber_ram::{MemoryObserver, Ram};

use crate::ppu_mode::PpuMode;

#[derive(Debug)]
pub struct PpuModeObserver {
    mode: PpuMode,
}

impl PpuModeObserver {
    pub const STAT_ADDRESS: u16 = 0xFF41;

    pub fn new() -> Self {
        Self {
            // Initial mode does not matter, it will be updated on add
            mode: PpuMode::HBlank,
        }
    }

    pub fn on_add(&mut self, ram: &mut Ram) {
        self.mode = Self::read_mode(ram);
        self.update_accessibility(ram);
    }

    pub fn read_mode(ram: &mut Ram) -> PpuMode {
        let value = ram.read(Self::STAT_ADDRESS);
        PpuMode::from_u8(value)
    }

    fn update_accessibility(&self, ram: &mut Ram) {
        self.update_vram_accessibility(ram);
        self.update_oam_accessibility(ram);
    }

    fn update_vram_accessibility(&self, ram: &mut Ram) {
        ram.set_vram_accessibility(self.mode != PpuMode::PixelTransfer);
    }

    fn update_oam_accessibility(&self, ram: &mut Ram) {
        let accessible = matches!(self.mode, PpuMode::OamScan | PpuMode::PixelTransfer);
        ram.set_oam_accessibility(accessible);
    }
}

impl MemoryObserver for PpuModeObserver {
    fn on_write(&mut self, ram: &mut Ram, address: u16, value: u8) {
        if address == Self::STAT_ADDRESS {
            self.mode = PpuMode::from_u8(value);
            self.update_accessibility(ram);
        }
    }
}

impl Default for PpuModeObserver {
    fn default() -> Self {
        Self::new()
    }
}
