mod mbc0;
mod mbc1;
mod mbc2;

pub use mbc0::Mbc0;
pub use mbc1::Mbc1;
pub use mbc2::Mbc2;

use crate::cartridges::{cartridge_mbc_info::{CartridgeMbcInfo, MbcType}, ExternalRamAddress};

pub trait Mbc {
    fn ram_enabled(&self) -> bool;
    fn rom_write(&mut self, address: u16, value: u8);
    fn rom_address(&self, address: u16) -> usize;
    fn ram_address(&self, address: u16) -> ExternalRamAddress;
}

pub enum MbcKind {
    Mbc0(Mbc0),
    Mbc1(Mbc1),
    Mbc2(Mbc2),
}

impl MbcKind {
    pub fn new(info: &CartridgeMbcInfo) -> Self {
        match info.mbc_type {
            MbcType::Mbc0 => MbcKind::Mbc0(Mbc0::new()),
            MbcType::Mbc1 => MbcKind::Mbc1(Mbc1::new(info.rom_bank_count, info.ram_bank_count)),
            MbcType::Mbc2 => MbcKind::Mbc2(Mbc2::new()),
            _ => unimplemented!("Unsupported MBC type: {:?}", info.mbc_type),
        }
    }
}

impl Mbc for MbcKind {
    fn ram_enabled(&self) -> bool {
        match self {
            MbcKind::Mbc0(mbc) => mbc.ram_enabled(),
            MbcKind::Mbc1(mbc) => mbc.ram_enabled(),
            MbcKind::Mbc2(mbc) => mbc.ram_enabled(),
        }
    }

    fn rom_write(&mut self, address: u16, value: u8) {
        match self {
            MbcKind::Mbc0(mbc) => mbc.rom_write(address, value),
            MbcKind::Mbc1(mbc) => mbc.rom_write(address, value),
            MbcKind::Mbc2(mbc) => mbc.rom_write(address, value),
        }
    }

    fn rom_address(&self, address: u16) -> usize {
        match self {
            MbcKind::Mbc0(mbc) => mbc.rom_address(address),
            MbcKind::Mbc1(mbc) => mbc.rom_address(address),
            MbcKind::Mbc2(mbc) => mbc.rom_address(address),
        }
    }

    fn ram_address(&self, address: u16) -> ExternalRamAddress {
        match self {
            MbcKind::Mbc0(mbc) => mbc.ram_address(address),
            MbcKind::Mbc1(mbc) => mbc.ram_address(address),
            MbcKind::Mbc2(mbc) => mbc.ram_address(address),
        }
    }
}

impl From<&CartridgeMbcInfo> for MbcKind {
    fn from(info: &CartridgeMbcInfo) -> Self {
        Self::new(info)
    }
}
