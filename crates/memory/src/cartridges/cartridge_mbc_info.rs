use crate::cartridges::cartridge_header::CartridgeHeader;

const MBC2_RAM_SIZE: usize = 0x200; // 512B

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MbcType {
    #[default]
    Mbc0,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Mbc6,
    Mbc7,
    Mmm01,
    HuC1,
    HuC3,
    Tama5,
    PocketCamera,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CartridgeMbcInfo {
    pub mbc_type: MbcType,
    pub rom_bank_count: usize,
    pub rom_size: usize,
    pub ram_bank_count: usize,
    pub ram_size: usize,
    pub includes_ram: bool,
    pub includes_battery: bool,
    pub includes_timer: bool,
    #[allow(dead_code)]
    pub includes_rumble: bool,
}

impl CartridgeMbcInfo {
    pub fn new(header: &CartridgeHeader) -> Self {
        let rom_bank_count = rom_bank_count(header.rom_size);
        let ram_bank_count = ram_bank_count(header.ram_size);
        let rom_size = rom_bank_count * 0x4000;
        let ram_size = ram_bank_count * 0x2000;

        match header.type_code {
            0x00 => Self {
                mbc_type: MbcType::Mbc0,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                ..Default::default()
            },
            0x01 => Self {
                mbc_type: MbcType::Mbc1,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                ..Default::default()
            },
            0x02 => Self {
                mbc_type: MbcType::Mbc1,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                ..Default::default()
            },
            0x03 => Self {
                mbc_type: MbcType::Mbc1,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                includes_battery: true,
                ..Default::default()
            },
            0x05 => Self {
                mbc_type: MbcType::Mbc2,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size: MBC2_RAM_SIZE,
                includes_ram: true,
                ..Default::default()
            },
            0x06 => Self {
                mbc_type: MbcType::Mbc2,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size: MBC2_RAM_SIZE,
                includes_ram: true,
                includes_battery: true,
                ..Default::default()
            },
            0x0B => Self {
                mbc_type: MbcType::Mmm01,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                ..Default::default()
            },
            0x0C => Self {
                mbc_type: MbcType::Mmm01,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                ..Default::default()
            },
            0x0D => Self {
                mbc_type: MbcType::Mmm01,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                includes_battery: true,
                ..Default::default()
            },
            0x0F => Self {
                mbc_type: MbcType::Mbc3,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_battery: true,
                includes_timer: true,
                ..Default::default()
            },
            0x10 => Self {
                mbc_type: MbcType::Mbc3,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                includes_battery: true,
                includes_timer: true,
                ..Default::default()
            },
            0x11 => Self {
                mbc_type: MbcType::Mbc3,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                ..Default::default()
            },
            0x12 => Self {
                mbc_type: MbcType::Mbc3,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                ..Default::default()
            },
            0x13 => Self {
                mbc_type: MbcType::Mbc3,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                includes_battery: true,
                ..Default::default()
            },
            0x19 => Self {
                mbc_type: MbcType::Mbc5,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                ..Default::default()
            },
            0x1A => Self {
                mbc_type: MbcType::Mbc5,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                ..Default::default()
            },
            0x1B => Self {
                mbc_type: MbcType::Mbc5,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                includes_battery: true,
                ..Default::default()
            },
            0x1C => Self {
                mbc_type: MbcType::Mbc5,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_rumble: true,
                ..Default::default()
            },
            0x1D => Self {
                mbc_type: MbcType::Mbc5,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                includes_rumble: true,
                ..Default::default()
            },
            0x1E => Self {
                mbc_type: MbcType::Mbc5,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                includes_battery: true,
                includes_rumble: true,
                ..Default::default()
            },
            0x20 => Self {
                mbc_type: MbcType::Mbc6,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                ..Default::default()
            },
            0x22 => Self {
                mbc_type: MbcType::Mbc7,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                includes_battery: true,
                includes_rumble: true,
                ..Default::default()
            },
            0xFC => Self {
                mbc_type: MbcType::PocketCamera,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                ..Default::default()
            },
            0xFD => Self {
                mbc_type: MbcType::Tama5,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                ..Default::default()
            },
            0xFE => Self {
                mbc_type: MbcType::HuC3,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                ..Default::default()
            },
            0xFF => Self {
                mbc_type: MbcType::HuC1,
                rom_bank_count,
                rom_size,
                ram_bank_count,
                ram_size,
                includes_ram: true,
                includes_battery: true,
                ..Default::default()
            },
            _ => panic!("Invalid cartridge type code: {}", header.type_code),
        }
    }
}

fn rom_bank_count(code: u8) -> usize {
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

fn ram_bank_count(code: u8) -> usize {
    match code {
        0x00 => 0,
        0x01 => unreachable!("RAM size 0x01 marked as unused"),
        0x02 => 1,  // 8KB, 1 bank of 8KB
        0x03 => 4,  // 32KB, 4 banks of 8KB
        0x04 => 16, // 128KB, 16 bank of 8KB
        0x05 => 8,  // 64KB, 8 bank of 8KB
        _ => unreachable!(),
    }
}
