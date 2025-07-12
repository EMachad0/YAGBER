use crate::{Bus, Memory, ram::Ram};

#[derive(Debug)]
pub struct Oam {
    ram: Ram,
    accessible: bool,
}

impl Oam {
    const SIZE: usize = 0xA0;
    const OFFSET: usize = 0xFE00;

    pub fn new() -> Self {
        Self {
            ram: Ram::new(Self::SIZE, Self::OFFSET),
            accessible: true,
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

    pub fn set_accessible(&mut self, accessible: bool) {
        self.accessible = accessible;
    }

    pub(crate) fn on_stat_write(bus: &mut Bus, value: u8) {
        let stat = super::Stat::new(value);
        let mode = stat.mode();
        bus.oam.set_accessible(mode != 2 && mode != 3);
    }
}

impl Memory for Oam {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}

impl Default for Oam {
    fn default() -> Self {
        Self::new()
    }
}
