#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct CartridgeHeader {
    pub entry_point: [u8; 4],
    pub logo: [u8; 48],
    pub title: String,
    pub cgb_flag: u8,
    pub licence_code: [u8; 2],
    pub sgb_flag: u8,
    pub type_code: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub destination_code: u8,
    pub old_license_code: u8,
    pub mask_rom_version: u8,
    pub checksum: u8,
    pub global_checksum: [u8; 2],
}

impl CartridgeHeader {
    const ENTRY_POINT: usize = 0x0100;
    const LOGO_ADR: usize = 0x0104;
    const TITLE_ADR: usize = 0x0134;
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
        Self {
            entry_point: Self::entry_point_from_rom(rom),
            logo: Self::logo_from_rom(rom),
            title: Self::title_from_rom(rom),
            cgb_flag: Self::cgb_flag_from_rom(rom),
            licence_code: Self::licence_code_from_rom(rom),
            sgb_flag: Self::sgb_flag_from_rom(rom),
            type_code: Self::type_code_from_rom(rom),
            rom_size: Self::rom_size_from_rom(rom),
            ram_size: Self::ram_size_from_rom(rom),
            destination_code: Self::destination_code_from_rom(rom),
            old_license_code: Self::old_license_code_from_rom(rom),
            mask_rom_version: Self::mask_rom_version_from_rom(rom),
            checksum: Self::checksum_from_rom(rom),
            global_checksum: Self::global_checksum_from_rom(rom),
        }
    }

    fn entry_point_from_rom(rom: &[u8]) -> [u8; 4] {
        rom[Self::ENTRY_POINT..Self::ENTRY_POINT + 4]
            .try_into()
            .unwrap()
    }

    fn logo_from_rom(rom: &[u8]) -> [u8; 48] {
        rom[Self::LOGO_ADR..Self::LOGO_ADR + 48].try_into().unwrap()
    }

    fn title_from_rom(rom: &[u8]) -> String {
        let title = &rom[Self::TITLE_ADR..Self::TITLE_ADR + 15];
        let title = title
            .iter()
            .take_while(|b| b.is_ascii_alphanumeric())
            .cloned()
            .collect::<Vec<_>>();
        String::from_utf8(title).unwrap()
    }

    fn cgb_flag_from_rom(rom: &[u8]) -> u8 {
        rom[Self::CGB_FLAG_ADR]
    }

    fn licence_code_from_rom(rom: &[u8]) -> [u8; 2] {
        rom[Self::LICENCE_CODE_ADR..Self::LICENCE_CODE_ADR + 2]
            .try_into()
            .unwrap()
    }

    fn sgb_flag_from_rom(rom: &[u8]) -> u8 {
        rom[Self::SGB_FLAG_ADR]
    }

    fn type_code_from_rom(rom: &[u8]) -> u8 {
        rom[Self::TYPE_ADR]
    }

    fn rom_size_from_rom(rom: &[u8]) -> u8 {
        rom[Self::ROM_SIZE_ADR]
    }

    fn ram_size_from_rom(rom: &[u8]) -> u8 {
        rom[Self::RAM_SIZE_ADR]
    }

    fn destination_code_from_rom(rom: &[u8]) -> u8 {
        rom[Self::DESTINATION_CODE_ADR]
    }

    fn old_license_code_from_rom(rom: &[u8]) -> u8 {
        rom[Self::OLD_LICENSE_CODE_ADR]
    }

    fn mask_rom_version_from_rom(rom: &[u8]) -> u8 {
        rom[Self::MASK_ROM_ADR]
    }

    fn checksum_from_rom(rom: &[u8]) -> u8 {
        rom[Self::CHECKSUM_ADR]
    }

    fn global_checksum_from_rom(rom: &[u8]) -> [u8; 2] {
        rom[Self::GLOBAL_CHECKSUM_ADR..Self::GLOBAL_CHECKSUM_ADR + 2]
            .try_into()
            .unwrap()
    }
}
