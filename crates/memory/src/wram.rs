use crate::{Bus, Memory, ram::Ram};

#[derive(Debug)]
pub struct Wram {
    // gbc wram has eight banks of 8kb each
    ram: [Ram; 8],
    current_bank: usize,
}

impl Wram {
    const SIZE: u16 = 0x1000;
    const OFFSET_BANK_0: u16 = 0xC000;
    const END_ADDRESS_BANK_0: u16 = Self::OFFSET_BANK_0 + Self::SIZE;
    const OFFSET_BANK_1: u16 = 0xD000;
    const END_ADDRESS_BANK_1: u16 = Self::OFFSET_BANK_1 + Self::SIZE;

    pub fn new() -> Self {
        let size = Self::SIZE as usize;
        let offset_bank_0 = Self::OFFSET_BANK_0 as usize;
        let offset_bank_1 = Self::OFFSET_BANK_1 as usize;
        Self {
            ram: [
                Ram::new(size, offset_bank_0),
                Ram::new(size, offset_bank_1),
                Ram::new(size, offset_bank_1),
                Ram::new(size, offset_bank_1),
                Ram::new(size, offset_bank_1),
                Ram::new(size, offset_bank_1),
                Ram::new(size, offset_bank_1),
                Ram::new(size, offset_bank_1),
            ],
            current_bank: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let current_bank = self.switchable_bank_idx();
        match address {
            Self::OFFSET_BANK_0..Self::END_ADDRESS_BANK_0 => self.ram[0].read(address),
            Self::OFFSET_BANK_1..Self::END_ADDRESS_BANK_1 => self.ram[current_bank].read(address),
            _ => unreachable!("Wram: read from invalid address: {}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let current_bank = self.switchable_bank_idx();
        match address {
            Self::OFFSET_BANK_0..Self::END_ADDRESS_BANK_0 => self.ram[0].write(address, value),
            Self::OFFSET_BANK_1..Self::END_ADDRESS_BANK_1 => {
                self.ram[current_bank].write(address, value)
            }
            _ => unreachable!("Wram: write to invalid address: {:X}", address),
        }
    }

    pub fn set_bank(&mut self, bank: usize) {
        self.current_bank = bank;
    }

    fn switchable_bank_idx(&self) -> usize {
        if self.current_bank == 0 {
            1
        } else {
            self.current_bank
        }
    }

    pub(crate) fn on_svbk_write(bus: &mut Bus, value: u8) {
        let bank = value & 0x07;
        bus.wram.set_bank(bank as usize);
    }
}

impl Memory for Wram {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}

impl Default for Wram {
    fn default() -> Self {
        Self::new()
    }
}
