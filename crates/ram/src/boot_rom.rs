use crate::memory::Memory;

#[derive(Debug, Clone)]
pub struct BootRom {
    data: Box<[Option<u8>]>,
}

impl BootRom {
    pub fn new() -> Self {
        let path = "resources/cgb_boot.bin";
        let data =
            std::fs::read(path).unwrap_or_else(|_| panic!("Failed to read boot ROM from {}", path));

        // CGB boot ROM is split into two parts
        // 0x0000–0x00FF: CGB boot ROM
        // 0x0100–0x08FF: CGB boot ROM (bank 0)
        // The cartridge Header is at 0x0100–0x014F (which is in the middle of the boot ROM)
        // On this boot room, the cartridge header starts as zeroes

        Self::from_bytes(&data)
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            data: data.iter().map(|&byte| Some(byte)).collect(),
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        if cfg!(feature = "break_on_unitialized_ram_read") && self.data[address].is_none() {
            panic!("Uninitialized ROM read at address: {:#X}", address);
        }
        self.data[address].unwrap_or_default()
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.data[address] = Some(value);
    }
}

impl Memory for BootRom {
    fn read(&self, address: u16) -> u8 {
        self.read(address as usize)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address as usize, value);
    }
}
