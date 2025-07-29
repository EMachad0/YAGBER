use crate::cartridges::save::SaveBackend;

#[derive(Default)]
pub struct MemoryBackend {
    buffer: Vec<u8>,
}

impl MemoryBackend {
    pub fn new(size: usize) -> Self {
        let buffer = vec![0; size];
        Self { buffer }
    }

    pub fn read(&self, address: usize) -> u8 {
        self.buffer[address]
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.buffer[address] = value;
    }
}

impl SaveBackend for MemoryBackend {
    fn read(&self, address: usize) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: usize, value: u8) {
        self.write(address, value);
    }
}
