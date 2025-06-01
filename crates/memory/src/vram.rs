use crate::{Memory, ram::Ram};

#[derive(Debug)]
pub struct Vram {
    ram: Ram,
    accessible: bool,
}

impl Vram {
    const SIZE: usize = 0x2000;
    const OFFSET: usize = 0x8000;

    pub fn new() -> Self {
        Self {
            ram: Ram::new(Self::SIZE, Self::OFFSET),
            accessible: false,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if self.accessible {
            self.ram.read(address)
        } else {
            0xFF
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if self.accessible {
            self.ram.write(address, value);
        }
    }

    pub(crate) fn set_accessible(&mut self, accessible: bool) {
        self.accessible = accessible;
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
