use crate::cartridges::saves::{SaveBackend, save::Save};

#[derive(Default)]
pub struct MemoryBackend;

impl SaveBackend for MemoryBackend {
    fn write(&mut self, _save: &super::save::Save) {
        // This is a no-op for the memory backend.
    }

    fn read(&mut self) -> super::save::Save {
        Save::default()
    }
}
