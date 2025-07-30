use std::{io::Read, os::unix::fs::FileExt};

use crate::cartridges::save::SaveBackend;

/// Save backend that stores data in a file system.
/// To be used with native targets that support file system.
pub struct NativeFileBackend {
    _path: std::path::PathBuf,
    buffer: Vec<u8>,
    file: std::fs::File,
    dirty: bool,
    size: usize,
}

impl NativeFileBackend {
    pub fn new(path: impl Into<std::path::PathBuf>, size: usize) -> Result<Self, std::io::Error> {
        let path = path.into();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)?;
        file.set_len(size as u64)?;

        let mut buffer = Vec::with_capacity(size);
        file.read_to_end(&mut buffer)?;

        Ok(Self {
            _path: path,
            buffer,
            file,
            dirty: false,
            size,
        })
    }

    fn read(&self, address: usize) -> u8 {
        self.buffer[address]
    }

    fn write(&mut self, address: usize, value: u8) {
        self.buffer[address] = value;
        self.dirty = true;
    }

    fn flush(&mut self) -> Result<usize, std::io::Error> {
        if self.dirty {
            self.dirty = false;
            self.file.write_all_at(&self.buffer, 0)?;
            Ok(self.size)
        } else {
            Ok(0)
        }
    }
}

impl SaveBackend for NativeFileBackend {
    fn read(&self, address: usize) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: usize, value: u8) {
        self.write(address, value)
    }
}

impl Drop for NativeFileBackend {
    fn drop(&mut self) {
        let _result = self.flush();
        #[cfg(feature = "trace")]
        if let Err(e) = _result {
            tracing::error!("Failed to flush save file: {}", e);
        }
    }
}
