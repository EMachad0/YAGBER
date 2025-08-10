use arbitrary_int::{u4, u7};

use crate::cartridges::{ExternalRamAddress, Mbc};

pub struct Mbc3 {
    ram_enabled: bool,
    rom_bank_number: u7,
    /// Ram bank number or Upper 2 bits of the rom bank number
    ram_bank_number: u4,
}

impl Mbc3 {
    pub fn new() -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: u7::from_u8(0),
            ram_bank_number: u4::from_u8(0),
        }
    }

    fn bank_0_address(&self, address: u16) -> usize {
        let offset = address as usize;
        offset & 0x3FFF
    }

    fn bank_1_address(&self, address: u16) -> usize {
        let bank = {
            let bank = self.rom_bank_number.value() as usize;
            if bank == 0 { 1 } else { bank }
        };

        let offset = address as usize;
        (bank * 0x4000) | (offset & 0x3FFF)
    }

    fn ram_address(&self, address: u16) -> ExternalRamAddress {
        let bank = self.ram_bank_number.value();
        if bank <= 0x07 {
            let bank = bank as usize;
            let offset = address as usize;
            let external_address = (bank * 0x2000) | (offset & 0x1FFF);
            ExternalRamAddress::ExternalRam(external_address)
        } else if bank <= 0x0C {
            use crate::cartridges::RtcRegisterKind::*;
            let register = match bank {
                0x08 => Seconds,
                0x09 => Minutes,
                0x0A => Hours,
                0x0B => DaysLow,
                0x0C => DaysHigh,
                _ => unreachable!()
            };
            ExternalRamAddress::Rtc(register)
        } else {
            unreachable!("invalid ram bank number for MBC3")
        }
    }
}

impl Mbc for Mbc3 {
    fn rom_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0A) == 0x0A;
            }
            0x2000..=0x3FFF => {
                self.rom_bank_number = u7::from_u8(value & 0x7F);
            }
            0x4000..=0x5FFF => {
                self.ram_bank_number = u4::from_u8(value & 0x03);
            }
            0x6000..=0x7FFF => {
                // Latch clock data - Not implemented
            }
            _ => unreachable!("Invalid address for MBC3 write: {address:#X}"),
        }
    }

    fn rom_address(&self, address: u16) -> usize {
        match address {
            0x0000..=0x3FFF => self.bank_0_address(address),
            0x4000..=0x7FFF => self.bank_1_address(address),
            _ => unreachable!("Invalid address for MBC3 ROM read: {address:#X}"),
        }
    }

    fn ram_address(&self, address: u16) -> ExternalRamAddress {
        self.ram_address(address)
    }

    fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }
}
