use std::{io::Read, os::unix::fs::FileExt};

use crate::cartridges::saves::{save::Save, SaveBackend};

/// Save backend that stores data in a file system.
/// To be used with native targets that support file system.
pub struct NativeFileBackend {
    _path: std::path::PathBuf,
    file: std::fs::File,
}

impl NativeFileBackend {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Result<Self, std::io::Error> {
        let path = path.into();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)?;

        Ok(Self {
            _path: path,
            file,
        })
    }
}

impl SaveBackend for NativeFileBackend {
    fn read(&mut self) -> Save {
        let mut bytes = Vec::new();
        let _ = self.file.read_to_end(&mut bytes);
        #[cfg(feature = "trace")]
        tracing::debug!("{bytes:?}");
        serde_json::from_slice(&bytes).unwrap_or_else(|_e| {
            #[cfg(feature = "trace")]
            tracing::error!("Unable to parse save file into save: {_e}");
            Save::default()
        })
    }

    fn write(&mut self, save: &super::save::Save) {
        let mut buf = Vec::new();
        if cfg!(debug_assertions) {
            #[cfg(feature = "trace")]
            tracing::debug!("Saving pretty");
            serde_json::to_writer_pretty(&mut buf, save)
        } else {
            serde_json::to_writer(&mut buf, save)
        }.expect("failed to serialize save");

        self.file.set_len(buf.len() as u64).expect("Unable to write file");
        self.file.write_all_at(&buf, 0).expect("Unable to write file")
    }
}
