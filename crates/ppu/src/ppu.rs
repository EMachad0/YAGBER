use yagber_ram::Ram;

use crate::{dot_clock::DotClock, scan_line::ScanLine};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ppu {
    scan_line: ScanLine,
    scan_line_index: u8,
    dot_clock: DotClock,
}

impl Ppu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, ram: &mut Ram) {
        self.dot_clock.tick();
        for _ in 0..self.dot_clock.times_finished_this_tick() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        self.scan_line.step();
        if self.scan_line.finished() {
            self.scan_line_index += 1;
            if self.scan_line_index >= 154 {
                self.scan_line_index = 0;
            }
            self.scan_line = ScanLine::new(self.scan_line_index);
        }
    }

    pub fn reset(&mut self) {
        self.scan_line_index = 0;
        self.scan_line = ScanLine::default();
    }
}
