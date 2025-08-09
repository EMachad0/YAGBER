use arbitrary_int::{u2, u5};

use crate::cartridges::{ExternalRamAddress, Mbc};

pub struct Mbc1 {
    ram_enabled: bool,
    rom_bank_number: u5,
    /// Ram bank number or Upper 2 bits of the rom bank number
    ram_bank_number: u2,
    rom_bank_count: usize,
    ram_bank_count: usize,
    mode: u8,
}

impl Mbc1 {
    pub fn new(rom_bank_count: usize, ram_bank_count: usize) -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: u5::from_u8(0),
            ram_bank_number: u2::from_u8(0),
            rom_bank_count,
            ram_bank_count,
            mode: 0,
        }
    }

    fn bank_0_address(&self, address: u16) -> usize {
        let bank = match self.mode {
            0 => 0x00,
            1 => self.ram_bank_number.value(),
            _ => unreachable!("Invalid mode: {}", self.mode),
        } as usize;

        let offset = address as usize;
        (bank * 0x4000) | (offset & 0x3FFF)
    }

    fn bank_1_address(&self, address: u16) -> usize {
        let mask = (1 << self.rom_bank_count) - 1;

        let bank = {
            let bank = self.rom_bank_number.value() as usize;
            if bank == 0 { 1 } else { bank }
        };

        let secondary_bank = self.ram_bank_number.value() as usize;
        let bank = bank | (secondary_bank << 5);
        let bank = bank & mask;

        let offset = address as usize;
        (bank * 0x4000) | (offset & 0x3FFF)
    }

    fn ram_address(&self, address: u16) -> ExternalRamAddress {
        let mask = (1 << self.ram_bank_count) - 1;
        let bank = match self.mode {
            0 => 0x00,
            1 => self.ram_bank_number.value(),
            _ => unreachable!("Invalid mode: {}", self.mode),
        } as usize;
        let bank = bank & mask;
        let offset = address as usize;
        let external_address = (bank * 0x2000) | (offset & 0x1FFF);
        ExternalRamAddress::ExternalRam(external_address)
    }
}

impl Mbc for Mbc1 {
    fn rom_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // Enable or disable RAM
                self.ram_enabled = (value & 0x0A) == 0x0A;
            }
            0x2000..=0x3FFF => {
                self.rom_bank_number = u5::from_u8(value & 0x1F);
            }
            0x4000..=0x5FFF => {
                self.ram_bank_number = u2::from_u8(value & 0x03);
            }
            0x6000..=0x7FFF => {
                // Set mode
                if value & 0x01 == 0x01 {
                    self.mode = 1;
                } else {
                    self.mode = 0;
                }
            }
            _ => unreachable!("Invalid address for MBC1 write: {:#X}", address),
        }
    }

    fn rom_address(&self, address: u16) -> usize {
        match address {
            0x0000..=0x3FFF => self.bank_0_address(address),
            0x4000..=0x7FFF => self.bank_1_address(address),
            _ => unreachable!("Invalid address for MBC1 ROM read: {:#X}", address),
        }
    }

    fn ram_address(&self, address: u16) -> ExternalRamAddress {
        self.ram_address(address)
    }

    fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }
}
