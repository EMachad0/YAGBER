use yagber_ram::Ram;

use crate::{ppu::Ppu, ppu_mode::PpuMode};

#[derive(Debug, Clone, Copy)]
pub struct ScanLine {
    dots: u32,
    finished: bool,
}

impl ScanLine {
    pub fn new() -> Self {
        Self {
            dots: 0,
            finished: false,
        }
    }

    pub fn step(&mut self, ram: &mut Ram) {
        self.dots += 1;
        let mode = Ppu::get_mode(ram);
        if self.dots >= mode.duration() {
            self.dots = 0;
            match mode {
                PpuMode::OamScan => Ppu::set_mode(ram, PpuMode::PixelTransfer),
                PpuMode::PixelTransfer => Ppu::set_mode(ram, PpuMode::HBlank),
                PpuMode::HBlank => self.finished = true,
                PpuMode::VBlank => self.finished = true,
            }
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}

impl Default for ScanLine {
    fn default() -> Self {
        Self::new()
    }
}
