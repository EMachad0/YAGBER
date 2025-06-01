use yagber_memory::{Bus, MemoryObserver};

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

    pub fn on_add(&mut self, bus: &mut Bus) {
        self.mode = Self::read_mode(bus);
        self.update_accessibility(bus);
    }

    pub fn read_mode(bus: &mut Bus) -> PpuMode {
        let value = bus.read(Self::STAT_ADDRESS);
        PpuMode::from_u8(value)
    }

    fn update_accessibility(&self, bus: &mut Bus) {
        self.update_vram_accessibility(bus);
        self.update_oam_accessibility(bus);
    }

    fn update_vram_accessibility(&self, bus: &mut Bus) {
        bus.set_vram_accessibility(self.mode != PpuMode::PixelTransfer);
    }

    fn update_oam_accessibility(&self, bus: &mut Bus) {
        let accessible = matches!(self.mode, PpuMode::OamScan | PpuMode::PixelTransfer);
        bus.set_oam_accessibility(accessible);
    }
}

impl MemoryObserver for PpuModeObserver {
    fn on_write(&mut self, bus: &mut Bus, address: u16, value: u8) {
        if address == Self::STAT_ADDRESS {
            self.mode = PpuMode::from_u8(value);
            self.update_accessibility(bus);
        }
    }
}

impl Default for PpuModeObserver {
    fn default() -> Self {
        Self::new()
    }
}
