const RAM_SIZE: usize = 0x10000; // 64 KiB (0x0000–0xFFFF)

pub struct Ram {
    data: [Option<u8>; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            data: [None; RAM_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if cfg!(feature = "break_on_unitialized_ram_read") {
            if self.data[address as usize].is_none() {
                panic!("Uninitialized RAM read at address: {:#X}", address);
            }
        }
        self.data[address as usize].unwrap_or_default()
    }

    pub fn copy_from_slice(&mut self, range: std::ops::Range<u16>, src: &[u8]) {
        let start = range.start as usize;
        let end = range.end as usize;
        if end > RAM_SIZE {
            panic!(
                "Attempt to write beyond RAM size: {:#06X}..{:#06X}",
                range.start, range.end
            );
        }
        let len = end - start;
        for (i, &b) in src.iter().take(len).enumerate() {
            self.data[start + i] = Some(b);
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = Some(value);
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
