use crate::{
    cartridges::{
        cartridge_mbc_info::CartridgeMbcInfo, mbc::MbcKind, saves::{Save, SaveBackend, SaveBackendKind}, CartridgeHeader, Mbc
    },
    ram::Ram,
};

#[derive(Default)]
pub enum Cartridge {
    #[default]
    Empty,
    Loaded {
        mbc: MbcKind,
        rom: Ram,
        ram: Ram,
        save_backend: SaveBackendKind,
    },
}

impl Cartridge {
    pub fn new(rom: &[u8]) -> Self {
        let header = CartridgeHeader::new(rom);
        let mbc_info = CartridgeMbcInfo::new(&header);

        // Check if the ROM size is valid
        if rom.len() < mbc_info.rom_size {
            panic!("ROM size is smaller than expected");
        }

        let mut save_backend = SaveBackendKind::new(&header, &mbc_info);
        let mut save = save_backend.read();
        save.data.resize(mbc_info.ram_size, 0);

        let mbc = MbcKind::new(&mbc_info);
        let rom = Ram::from_bytes(rom, 0);
        let ram = Ram::from_bytes(&save.data, 0);

        Self::Loaded { mbc, rom, ram, save_backend }
    }

    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        match self {
            Self::Empty => {
                #[cfg(feature = "trace")]
                tracing::warn!("Reading from empty cartridge ROM");
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
                #[cfg(feature = "trace")]
                tracing::warn!("Reading from empty cartridge RAM");
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
            _ => panic!("Invalid address: {address:#X}"),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.write_rom(address, value),
            0xA000..=0xBFFF => self.write_ram(address, value),
            _ => panic!("Invalid address: {address:#X}"),
        }
    }
}

impl Drop for Cartridge {
    fn drop(&mut self) {
        match self {
            Cartridge::Empty => {},
            Cartridge::Loaded { ram, save_backend, .. } => {
                let timestamp = chrono::Utc::now().timestamp();
                let save = Save { data: ram.to_vec(), timestamp  };
                save_backend.write(&save);
            },
        }
    }
}

impl std::fmt::Debug for Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("Empty Cartridge"),
            Self::Loaded { .. } => f.write_str("Loaded Cartridge"),
        }
    }
}
