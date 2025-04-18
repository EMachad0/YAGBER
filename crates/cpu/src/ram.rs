const RAM_SIZE: usize = 0x10000; // 64 KiB (0x0000–0xFFFF)

pub struct Ram {
    pub data: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        let mut ram = Self {
            data: [0; RAM_SIZE],
        };

        // mirror I/O defaults in RAM
        ram.write(0xFF05, 0x00); // TIMA
        ram.write(0xFF06, 0x00); // TMA
        ram.write(0xFF07, 0x00); // TAC
        ram.write(0xFF0F, 0xE1); // IF
        ram.write(0xFFFF, 0x00); // IE

        ram
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
