use crate::cartridges::{
    CartridgeHeader,
    cartridge_mbc_info::CartridgeMbcInfo,
    save::{MemoryBackend, NativeFileBackend},
};

pub trait SaveBackend {
    fn read(&self, address: usize) -> u8;
    fn write(&mut self, address: usize, value: u8);
}

pub enum SaveBackendKind {
    Memory(MemoryBackend),
    NativeFile(NativeFileBackend),
}

impl SaveBackendKind {
    pub fn new(cartridge_header: &CartridgeHeader, mbc_info: &CartridgeMbcInfo) -> Self {
        if !mbc_info.battery_backed_ram {
            Self::Memory(MemoryBackend::new(mbc_info.ram_size))
        } else if cfg!(feature = "native") {
            Self::NativeFile(
                NativeFileBackend::new(
                    format!("out/saves/{}.sav", cartridge_header.title),
                    mbc_info.ram_size,
                )
                .unwrap(),
            )
        } else {
            panic!("Battery backed RAM is not supported on this platform");
        }
    }
}

impl SaveBackend for SaveBackendKind {
    fn read(&self, address: usize) -> u8 {
        match self {
            SaveBackendKind::Memory(backend) => backend.read(address),
            SaveBackendKind::NativeFile(backend) => backend.read(address),
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        match self {
            SaveBackendKind::Memory(backend) => backend.write(address, value),
            SaveBackendKind::NativeFile(backend) => backend.write(address, value),
        }
    }
}
