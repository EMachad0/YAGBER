const RAM_SIZE: usize = 0xFFFF;

pub struct Ram {
    pub data: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            data: [0; RAM_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let lo = self.read(address);
        let hi = self.read(address + 1);
        u16::from_le_bytes([lo, hi])
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write(address, lo);
        self.write(address + 1, hi);
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}
