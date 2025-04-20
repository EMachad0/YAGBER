use crate::InterruptType;

const RAM_SIZE: usize = 0x10000; // 64 KiB (0x0000–0xFFFF)
// const BOOT_ROM_SIZE: usize = 0x8000; // 32 KiB (0x0000–0x7FFF)

#[derive(Debug, Clone)]
pub struct Ram {
    data: [Option<u8>; RAM_SIZE],
    boot_rom: Vec<u8>,
}

impl Ram {
    pub fn new() -> Self {
        let mut ram = Self {
            data: [None; RAM_SIZE],
            boot_rom: Vec::new(),
        };

        // Initialize BOOT ROM
        ram.with_boot_rom();

        // Initialize IO registers to 0x00
        // These may be read before they are written to by the boot ROM
        for i in 0xFF00..=0xFFFF {
            ram.data[i as usize] = Some(0x00);
        }

        ram
    }

    pub fn with_boot_rom(&mut self) {
        let path = "resources/cgb_boot.bin";
        let boot_rom =
            std::fs::read(path).unwrap_or_else(|_| panic!("Failed to read boot ROM from {}", path));

        // CGB boot ROM is split into two parts
        // 0x0000–0x00FF: CGB boot ROM
        // 0x0100–0x08FF: CGB boot ROM (bank 0)
        // The cartridge Header is at 0x0100–0x014F (which is in the middle of the boot ROM)
        // On this boot room, the cartridge header starts as zeroes
        // Copy the boot ROM to the RAM
        self.boot_rom = boot_rom;
    }

    pub fn read(&self, address: u16) -> u8 {
        if cfg!(feature = "break_on_unitialized_ram_read") && self.data[address as usize].is_none()
        {
            panic!("Uninitialized RAM read at address: {:#X}", address);
        }
        if self.booting() && address < 0x8000 && !(0x0100..0x014F).contains(&address) {
            // Read from boot ROM
            self.boot_rom[address as usize]
        } else {
            self.data[address as usize].unwrap_or_default()
        }
    }

    pub fn read_masked(&self, address: u16, mask: u8) -> u8 {
        let value = self.read(address);
        value & mask
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
        let len = (end - start).min(src.len());
        for i in 0..len {
            self.data[start + i] = Some(src[i]);
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = Some(value);
    }

    pub fn write_masked(&mut self, address: u16, value: u8, mask: u8) {
        let current_value = self.read(address);
        let new_value = (current_value & !mask) | (value & mask);
        self.write(address, new_value);
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

    pub fn set_bit(&mut self, address: u16, bit: u8) {
        let value = self.read(address);
        let new_value = value | (1 << bit);
        self.write(address, new_value);
    }

    pub fn request_interrupt(&mut self, interrupt: InterruptType) {
        self.set_bit(0xFF0F, interrupt.to_u8());
    }

    pub fn booting(&self) -> bool {
        // Directly read to avoid infinite loop
        match self.data[0xFF50] {
            Some(0x00) | None => true,
            _ => false,
        }
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}
