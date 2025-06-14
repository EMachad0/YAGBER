#![allow(dead_code)]

use crate::{
    Memory,
    mbc::{Mbc, Mbc0, Mbc1},
    ram::Ram,
};

#[derive(Default)]
pub enum Cartridge {
    #[default]
    Empty,
    Loaded {
        mbc: Box<dyn Mbc>,
        rom: Ram,
        ram: Ram,
    },
}

impl Cartridge {
    const ENTRY_POINT: usize = 0x0100;
    const LOGOO_ADR: usize = 0x0104;
    const TITLE_ADR: usize = 0x0134;
    const MANUFACTURER_ADR: usize = 0x013F;
    const CGB_FLAG_ADR: usize = 0x0143;
    const LICENCE_CODE_ADR: usize = 0x0144;
    const SGB_FLAG_ADR: usize = 0x0146;
    const TYPE_ADR: usize = 0x0147;
    const ROM_SIZE_ADR: usize = 0x0148;
    const RAM_SIZE_ADR: usize = 0x0149;
    const DESTINATION_CODE_ADR: usize = 0x014A;
    const OLD_LICENSE_CODE_ADR: usize = 0x014B;
    const MASK_ROM_ADR: usize = 0x014D;
    const CHECKSUM_ADR: usize = 0x014C;
    const GLOBAL_CHECKSUM_ADR: usize = 0x014E;

    pub fn new(rom: &[u8]) -> Self {
        let rom_code = rom[Self::ROM_SIZE_ADR];
        let rom_bank_count = decode_rom_size(rom_code);
        let rom_size = rom_bank_count * 0x4000; // 16KB per bank

        // Check if the ROM size is valid
        if rom.len() < rom_size {
            panic!("ROM size is smaller than expected");
        }

        let ram_code = rom[Self::RAM_SIZE_ADR];
        let ram_bank_count = decode_ram_size(ram_code);
        let ram_size = ram_bank_count * 0x2000; // 8KB per bank

        let mbc: Box<dyn Mbc> = match rom[Self::TYPE_ADR] {
            0x00 => Box::new(Mbc0::new()),
            0x01..=0x03 => Box::new(Mbc1::new(rom_bank_count, ram_bank_count)),
            _ => panic!("Unsupported MBC type"),
        };

        Self::Loaded {
            mbc,
            rom: Ram::from_bytes(rom, 0),
            ram: Ram::new(ram_size, 0),
        }
    }

    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        match self {
            Self::Empty => {
                warn!("Reading from empty cartridge ROM");
                0xFF
            }
            Self::Loaded { mbc, rom, .. } => {
                let address = mbc.rom_address(address);
                rom.read_usize(address)
            }
        }
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        match self {
            Self::Empty => (),
            Self::Loaded { mbc, .. } => mbc.rom_write(address, value),
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        match self {
            Self::Empty => {
                warn!("Reading from empty cartridge RAM");
                0xFF
            }
            Self::Loaded { mbc, ram, .. } => {
                if !mbc.ram_enabled() {
                    return 0xFF;
                }
                let address = mbc.ram_address(address);
                ram.read_usize(address)
            }
        }
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        match self {
            Self::Empty => (),
            Self::Loaded { mbc, ram, .. } => {
                if !mbc.ram_enabled() {
                    return;
                }
                let address = mbc.ram_address(address);
                ram.write_usize(address, value);
            }
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.read_rom(address),
            0xA000..=0xBFFF => self.read_ram(address),
            _ => panic!("Invalid address: {:#X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.write_rom(address, value),
            0xA000..=0xBFFF => self.write_ram(address, value),
            _ => panic!("Invalid address: {:#X}", address),
        }
    }
}

impl std::fmt::Debug for Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("Empty Cartridge"),
            Self::Loaded { rom, ram, .. } => f
                .debug_struct("Loaded Cartridge")
                .field("rom_size", &rom.len())
                .field("ram_size", &ram.len())
                .finish(),
        }
    }
}

impl Memory for Cartridge {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}

fn decode_rom_size(code: u8) -> usize {
    match code {
        0x00 => 2,   // 32KB, 2 banks (no banking)
        0x01 => 4,   // 64KB, 4 banks
        0x02 => 8,   // 128KB, 8 banks
        0x03 => 16,  // 256KB, 16 banks
        0x04 => 32,  // 512KB, 32 banks
        0x05 => 64,  // 1MB, 64 banks
        0x06 => 128, // 2MB, 128 banks
        0x07 => 256, // 4MB, 256 banks
        0x08 => 512, // 8MB, 512 banks
        0x52..=0x54 => unimplemented!("weird rom size"),
        _ => unreachable!(),
    }
}

fn decode_ram_size(code: u8) -> usize {
    match code {
        0x00 => 0,
        0x01 => unreachable!("MBC1 RAM size 0x01 marked as unused"),
        0x02 => 1,  // 8KB, 1 bank of 8KB
        0x03 => 4,  // 32KB, 4 banks of 8KB
        0x04 => 16, // 128KB, 16 bank of 8KB
        0x05 => 8,  // 64KB, 8 bank of 8KB
        _ => unreachable!(),
    }
}
