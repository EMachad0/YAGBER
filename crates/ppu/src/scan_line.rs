use crate::mode::Mode;

#[derive(Debug, Clone, Copy)]
pub struct ScanLine {
    dots: u32,
    mode: Mode,
    finished: bool,
}

impl ScanLine {
    pub fn new(index: u8) -> Self {
        Self {
            dots: 0,
            mode: match index {
                0..=143 => Mode::OamScam,
                144..153 => Mode::VBlank,
                _ => unreachable!(),
            },
            finished: false,
        }
    }

    pub fn duration(&self) -> u32 {
        self.mode.duration()
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn step(&mut self) {
        self.dots += 1;
        if self.dots >= self.mode.duration() {
            self.dots = 0;
            match self.mode {
                Mode::OamScam => self.mode = Mode::PixelTransfer,
                Mode::PixelTransfer => self.mode = Mode::HBlank,
                Mode::HBlank => self.finished = true,
                Mode::VBlank => self.finished = true,
            }
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}

impl Default for ScanLine {
    fn default() -> Self {
        Self::new(0)
    }
}
