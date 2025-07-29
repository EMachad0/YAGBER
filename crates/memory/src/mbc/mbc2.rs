use arbitrary_int::u4;

use crate::mbc::Mbc;

pub struct Mbc2 {
    ram_enabled: bool,
    rom_bank_number: u4,
}

impl Mbc2 {
    pub fn new() -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: u4::from_u8(0),
        }
    }

    fn bank_1_address(&self, address: u16) -> usize {
        let bank = {
            let bank = self.rom_bank_number.value() as usize;
            if bank == 0 { 1 } else { bank }
        };

        let offset = address as usize;
        (bank * 0x4000) | (offset & 0x3FFF)
    }

    fn ram_address(&self, address: u16) -> usize {
        (address & 0x1FF) as usize
    }
}

impl Mbc for Mbc2 {
    fn rom_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..0x4000 => {
                if address & 0x100 == 0 {
                    let ram_enabled = value & 0x0F == 0x0A;
                    self.ram_enabled = ram_enabled;
                } else {
                    let rom_bank_number = value & 0x0F;
                    self.rom_bank_number = u4::from_u8(rom_bank_number);
                }
            }
            _ => unreachable!("Invalid address for MBC2 write: {:#X}", address),
        }
    }

    fn rom_address(&self, address: u16) -> usize {
        match address {
            0x0000..=0x3FFF => address as usize,
            0x4000..=0x7FFF => self.bank_1_address(address),
            _ => unreachable!("Invalid address for MBC2 ROM read: {:#X}", address),
        }
    }

    fn ram_address(&self, address: u16) -> usize {
        self.ram_address(address)
    }

    fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }
}
