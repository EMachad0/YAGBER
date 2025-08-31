use arbitrary_int::{u4, u9};

use crate::cartridges::{ExternalRamAddress, Mbc, external_ram_address::MbcDeviceUpdate};

pub struct Mbc5 {
    ram_enabled: bool,
    rom_bank_number: u9,
    ram_bank_number: u4,
    rom_bank_count: usize,
    ram_bank_count: usize,
    includes_rumble: bool,
}

impl Mbc5 {
    pub fn new(rom_bank_count: usize, ram_bank_count: usize, includes_rumble: bool) -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: u9::from_u8(0),
            ram_bank_number: u4::from_u8(0),
            rom_bank_count,
            ram_bank_count,
            includes_rumble,
        }
    }

    fn bank_0_address(&self, address: u16) -> usize {
        (address as usize) & 0x3FFF
    }

    fn bank_1_address(&self, address: u16) -> usize {
        // Limit bank to available ROM size (rom_bank_count is power-of-two)
        let available_banks_mask = self.rom_bank_count.saturating_sub(1);
        let selected_bank = (self.rom_bank_number.value() as usize) & available_banks_mask;
        let offset_within_bank = (address as usize) & 0x3FFF;
        (selected_bank * 0x4000) | offset_within_bank
    }

    fn ram_ext_address(&self, address: u16) -> ExternalRamAddress {
        let available_ram_banks_mask = self.ram_bank_count.saturating_sub(1);
        let selected_ram_bank = (self.ram_bank_number.value() as usize) & available_ram_banks_mask;
        let offset_within_ram_bank = (address as usize) & 0x1FFF;
        let external_address = (selected_ram_bank * 0x2000) | offset_within_ram_bank;
        ExternalRamAddress::ExternalRam(external_address)
    }
}

impl Mbc for Mbc5 {
    fn rom_write(&mut self, address: u16, value: u8) -> Option<MbcDeviceUpdate> {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0A) == 0x0A;
            }
            0x2000..=0x2FFF => {
                let upper_bit = self.rom_bank_number.value() & 0x0100;
                let combined = (upper_bit) | (value as u16);
                self.rom_bank_number = u9::from_u16(combined);
            }
            0x3000..=0x3FFF => {
                let upper_bit = (value & 0x01) as u16;
                let lower = self.rom_bank_number.value() & 0x00FF;
                let combined = (upper_bit << 8) | lower;
                self.rom_bank_number = u9::from_u16(combined);
            }
            0x4000..=0x5FFF => {
                // RAM bank number. If cartridge has rumble, bit 3 is rumble control and must not
                // be used for banking; otherwise, all lower 4 bits select the RAM bank.
                let mask = if self.includes_rumble { 0x07 } else { 0x0F };
                self.ram_bank_number = u4::from_u8(value & mask);
                if self.includes_rumble {
                    let rumble_enabled = (value & 0x08) != 0;
                    return Some(MbcDeviceUpdate::RumbleMotor(rumble_enabled));
                }
            }
            0x6000..=0x7FFF => {}
            _ => unreachable!("Invalid address for MBC5 write: {address:#X}"),
        }
        None
    }

    fn rom_address(&self, address: u16) -> usize {
        match address {
            0x0000..=0x3FFF => self.bank_0_address(address),
            0x4000..=0x7FFF => self.bank_1_address(address),
            _ => unreachable!("Invalid address for MBC5 ROM read: {address:#X}"),
        }
    }

    fn ram_address(&self, address: u16) -> ExternalRamAddress {
        self.ram_ext_address(address)
    }

    fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }
}
