use crate::memory::Memory;

const ROM_SIZE: usize = 0x8000;

#[derive(Debug, Clone, Copy)]
pub struct Rom {
    data: [Option<u8>; ROM_SIZE],
}

impl Rom {
    pub fn new() -> Self {
        Self {
            data: [None; ROM_SIZE],
        }
    }

    pub fn boot_rom() -> Self {
        let path = "resources/cgb_boot.bin";
        let boot_rom =
            std::fs::read(path).unwrap_or_else(|_| panic!("Failed to read boot ROM from {}", path));

        // CGB boot ROM is split into two parts
        // 0x0000–0x00FF: CGB boot ROM
        // 0x0100–0x08FF: CGB boot ROM (bank 0)
        // The cartridge Header is at 0x0100–0x014F (which is in the middle of the boot ROM)
        // On this boot room, the cartridge header starts as zeroes

        let mut rom = Self::new();
        for (i, &byte) in boot_rom.iter().enumerate() {
            // Skip the cartridge header
            if (0x0100..=0x014F).contains(&(i as u16)) {
                continue;
            }
            rom.data[i] = Some(byte);
        }
        rom
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.data[i] = Some(byte);
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if cfg!(feature = "break_on_unitialized_ram_read") && self.data[address as usize].is_none()
        {
            panic!("Uninitialized RAM read at address: {:#X}", address);
        }
        self.data[address as usize].unwrap_or_default()
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = Some(value);
    }
}

impl Default for Rom {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for Rom {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}
