use crate::{Bus, Memory, ram::Ram};

#[derive(Debug)]
pub struct Vram {
    // gbc vram has two banks of 8kb each
    ram: [Ram; 2],
    current_bank: usize,
    accessible: bool,
}

impl Vram {
    const SIZE: usize = 0x2000;
    const OFFSET: usize = 0x8000;

    pub fn new() -> Self {
        // Initialise VRAM with 0xFF just like on real hardware after power-up
        // so that early reads (e.g. during boot ROM execution) don't
        // trigger the `break_on_unitialized_ram_read` debug assertion.
        let blank = vec![0xFF; Self::SIZE];
        Self {
            ram: [
                Ram::from_bytes(&blank, Self::OFFSET),
                Ram::from_bytes(&blank, Self::OFFSET),
            ],
            current_bank: 0,
            accessible: true,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if self.accessible {
            self.ram[self.current_bank].read(address)
        } else {
            0xFF
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if self.accessible {
            self.ram[self.current_bank].write(address, value);
        }
    }

    pub fn set_accessible(&mut self, accessible: bool) {
        self.accessible = accessible;
    }

    pub fn set_bank(&mut self, bank: usize) {
        self.current_bank = bank;
    }

    pub fn tile(&self, bank: bool, address: u16) -> &[Option<u8>] {
        let bank = if bank { 1 } else { 0 };
        let address = address as usize - Self::OFFSET;
        &self.ram[bank].data_slice()[address..address + 16]
    }

    pub fn tile_map(&self, lcdc3: bool) -> &[Option<u8>] {
        let (start, end) = self.get_map_range(lcdc3);
        &self.ram[0].data_slice()[start..end]
    }

    pub fn attr_map(&self, lcdc3: bool) -> &[Option<u8>] {
        let (start, end) = self.get_map_range(lcdc3);
        &self.ram[1].data_slice()[start..end]
    }

    fn get_map_range(&self, lcdc3: bool) -> (usize, usize) {
        let (start, end) = if !lcdc3 {
            (0x9800, 0x9C00)
        } else {
            (0x9C00, 0xA000)
        };
        (start - Self::OFFSET, end - Self::OFFSET)
    }

    pub(crate) fn on_vbk_write(bus: &mut Bus, value: u8) {
        let bank = value & 0x01;
        let vram = &mut bus.vram;
        vram.set_bank(bank as usize);
    }

    pub(crate) fn on_stat_write(bus: &mut Bus, value: u8) {
        let stat = super::Stat::new(value);
        let mode = stat.mode();
        bus.vram.set_accessible(mode != 3);
    }
}

impl Memory for Vram {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}

impl Default for Vram {
    fn default() -> Self {
        Self::new()
    }
}
