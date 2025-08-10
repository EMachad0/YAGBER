use crate::{
    cartridges::{
        CartridgeHeader, Mbc, Rtc,
        cartridge_mbc_info::CartridgeMbcInfo,
        mbc::MbcKind,
        saves::{Save, SaveBackend, SaveBackendKind},
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
        ram: Option<Ram>,
        rtc: Option<Rtc>,
        save_backend: SaveBackendKind,
    },
}

impl Cartridge {
    pub fn new(rom: &[u8]) -> Self {
        let header = CartridgeHeader::new(rom);
        let mbc_info = CartridgeMbcInfo::new(&header);

        #[cfg(feature = "trace")]
        tracing::debug!("{mbc_info:?}");

        // Check if the ROM size is valid
        if rom.len() < mbc_info.rom_size {
            panic!("ROM size is smaller than expected");
        }

        let mut save_backend = SaveBackendKind::new(&header, &mbc_info);
        let save = save_backend.read();

        let mbc = MbcKind::new(&mbc_info);
        let rom = Ram::from_bytes(rom, 0);
        let ram = if mbc_info.includes_ram {
            let mut save_data = save.data.unwrap_or_default();
            save_data.resize(mbc_info.ram_size, 0);
            Some(Ram::from_bytes(&save_data, 0))
        } else {
            None
        };
        let rtc = if mbc_info.includes_timer {
            let now_seconds = chrono::Utc::now().timestamp();
            let seconds_since_save = now_seconds - save.timestamp;
            let rtc_registers = match save.rtc_registers {
                Some(mut regs) => {
                    if seconds_since_save > 0 && !regs.halted() {
                        regs.advance_by(seconds_since_save as u64);
                    }
                    regs
                }
                None => crate::cartridges::rtc::RtcRegisters::default(),
            };
            Some(Rtc::from_registers(rtc_registers))
        } else {
            None
        };

        Self::Loaded {
            mbc,
            rom,
            ram,
            rtc,
            save_backend,
        }
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
            Self::Loaded { mbc, rtc, .. } => {
                if (0x6000..=0x7FFF).contains(&address) {
                    if let Some(rtc) = rtc.as_mut() {
                        rtc.latch_write(value);
                    }
                }
                mbc.rom_write(address, value)
            }
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        match self {
            Self::Empty => {
                #[cfg(feature = "trace")]
                tracing::warn!("Reading from empty cartridge RAM");
                0xFF
            }
            Self::Loaded { mbc, ram, rtc, .. } => {
                if !mbc.ram_enabled() {
                    return 0xFF;
                }
                match mbc.ram_address(address) {
                    super::ExternalRamAddress::ExternalRam(address) => match ram {
                        Some(ram) => ram.read_usize(address),
                        None => 0xFF,
                    },
                    super::ExternalRamAddress::Rtc(rtc_register_kind) => {
                        match rtc.as_ref() {
                            Some(rtc_ref) => rtc_ref.read_register(rtc_register_kind),
                            None => 0xFF,
                        }
                    }
                }
            }
        }
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        match self {
            Self::Empty => (),
            Self::Loaded { mbc, ram, rtc, .. } => {
                if !mbc.ram_enabled() {
                    return;
                }
                match mbc.ram_address(address) {
                    super::ExternalRamAddress::ExternalRam(address) => {
                        if let Some(ram) = ram {
                            ram.write_usize(address, value);
                        }
                    }
                    super::ExternalRamAddress::Rtc(rtc_register_kind) => {
                        if let Some(rtc) = rtc {
                            rtc.tick();
                            rtc.write_register(rtc_register_kind, value);
                        }
                    }
                }
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

    pub fn tick(&mut self) {
        let Cartridge::Loaded { rtc, .. } = self else {
            return;
        };

        if let Some(rtc) = rtc {
            rtc.tick();
        }
    }
}

impl Drop for Cartridge {
    fn drop(&mut self) {
        match self {
            Cartridge::Empty => {}
            Cartridge::Loaded {
                ram,
                rtc,
                save_backend,
                ..
            } => {
                let timestamp = chrono::Utc::now().timestamp();
                let data = ram.as_ref().map(|r| r.to_vec());
                let rtc_registers = rtc.as_mut().map(|rtc| {
                    rtc.tick();
                    rtc.registers
                });
                let save = Save {
                    data,
                    rtc_registers,
                    timestamp,
                };
                save_backend.write(&save);
            }
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
