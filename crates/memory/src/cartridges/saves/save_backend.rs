use crate::cartridges::{
    CartridgeHeader,
    cartridge_mbc_info::CartridgeMbcInfo,
    saves::{MemoryBackend, NativeFileBackend, save::Save},
};

pub trait SaveBackend {
    fn read(&mut self) -> Save;
    fn write(&mut self, save: &Save);
}

pub enum SaveBackendKind {
    Memory(MemoryBackend),
    NativeFile(NativeFileBackend),
}

impl SaveBackendKind {
    pub fn new(cartridge_header: &CartridgeHeader, mbc_info: &CartridgeMbcInfo) -> Self {
        if !mbc_info.includes_battery {
            Self::Memory(MemoryBackend)
        } else if cfg!(feature = "native") {
            #[cfg(feature = "trace")]
            tracing::info!("Saving to {}", cartridge_header.title,);
            let path = format!("out/saves/{}.sav", cartridge_header.title);
            Self::NativeFile(NativeFileBackend::new(path).unwrap())
        } else {
            panic!("Battery backed RAM is not supported on this platform");
        }
    }
}

impl SaveBackend for SaveBackendKind {
    fn read(&mut self) -> Save {
        match self {
            SaveBackendKind::Memory(memory_backend) => memory_backend.read(),
            SaveBackendKind::NativeFile(native_file_backend) => native_file_backend.read(),
        }
    }

    fn write(&mut self, save: &Save) {
        match self {
            SaveBackendKind::Memory(memory_backend) => memory_backend.write(save),
            SaveBackendKind::NativeFile(native_file_backend) => native_file_backend.write(save),
        }
    }
}
