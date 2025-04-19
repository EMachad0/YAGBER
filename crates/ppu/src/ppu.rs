use crate::{dot_clock::DotClock, scan_line::ScanLine};

pub struct Ppu {
    scan_line: ScanLine,
    scan_line_index: u8,
    dot_clock: DotClock,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            scan_line: ScanLine::default(),
            scan_line_index: 0,
            dot_clock: DotClock::new(),
        }
    }

    pub fn tick(&mut self) {
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
