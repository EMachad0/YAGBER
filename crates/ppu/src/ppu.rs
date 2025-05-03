use yagber_ram::{Memory, Ram};

use crate::{mode::Mode, scan_line::ScanLine};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ppu {
    scan_line: ScanLine,
}

impl Ppu {
    pub const SCAN_LINE_ADDRESS: u16 = 0xFF44;
    pub const LCD_STATUS_ADDRESS: u16 = 0xFF41;
    pub const LCD_CONTROL_ADDRESS: u16 = 0xFF40;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self, ram: &mut Ram) {
        self.scan_line.step(ram);
        if self.scan_line.finished() {
            let scan_line_index = Self::scan_line_index(ram);
            if scan_line_index >= 153 {
                Self::set_scan_line_index(ram, 0);
            } else {
                Self::set_scan_line_index(ram, scan_line_index + 1);
            }
            self.scan_line = ScanLine::new();
        }
    }

    pub fn enabled(ram: &Ram) -> bool {
        let lcd_control = ram.read(Self::LCD_CONTROL_ADDRESS);
        (lcd_control & 0x80) != 0
    }

    pub fn scan_line_index(ram: &mut Ram) -> u8 {
        ram.read(Self::SCAN_LINE_ADDRESS)
    }

    pub fn set_scan_line_index(ram: &mut Ram, index: u8) {
        match index {
            0..=143 => Ppu::set_mode(ram, Mode::OamScan),
            144..=153 => Ppu::set_mode(ram, Mode::VBlank),
            _ => panic!("Invalid scan line index: {}", index),
        }
        ram.write(Self::SCAN_LINE_ADDRESS, index);
    }

    pub fn get_mode(ram: &mut Ram) -> Mode {
        let mode = ram.read_masked(Ppu::LCD_STATUS_ADDRESS, 0x03);
        match mode {
            0 => Mode::OamScan,
            1 => Mode::PixelTransfer,
            2 => Mode::HBlank,
            3 => Mode::VBlank,
            _ => unreachable!(),
        }
    }

    pub fn set_mode(ram: &mut Ram, mode: Mode) {
        match mode {
            Mode::OamScan => {}
            Mode::PixelTransfer => {}
            Mode::HBlank => {}
            Mode::VBlank => {
                trace!("VBlank interrupt");
                ram.request_interrupt(yagber_ram::InterruptType::VBlank);
            }
        }
        ram.write_masked(Ppu::LCD_STATUS_ADDRESS, mode.to_u8(), 0x03);
    }
}
