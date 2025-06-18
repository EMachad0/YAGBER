use crate::{
    InterruptType, boot_rom::BootRom, cartridge::Cartridge, io_registers::IOBus, memory::Memory,
    oam::Oam, ram::Ram, register::Register, vram::Vram, wram::Wram,
};

#[derive(Debug)]
pub struct Bus {
    boot_rom: BootRom,
    cartridge: Cartridge,
    vram: Vram,
    wram: Wram,
    oam: Oam,
    io_registers: IOBus,
    hram: Ram,
    ie: Register,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            boot_rom: BootRom::new(),
            cartridge: Cartridge::empty(),
            vram: Vram::new(),
            wram: Wram::new(),
            oam: Oam::new(),
            io_registers: IOBus::new(),
            hram: Ram::new(0x7F, 0xFF80),
            ie: Register::new(0x00),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            // May be a boot ROM
            0x0000..=0x7FFF => self.read_rom(address),
            // VRAM
            0x8000..=0x9FFF => self.vram.read(address),
            // External RAM
            0xA000..=0xBFFF => self.cartridge.read(address),
            // WRAM
            0xC000..=0xDFFF => self.wram.read(address),
            // Echo RAM
            0xE000..=0xFDFF => self.wram.read(address - 0x2000),
            // OAM
            0xFE00..=0xFE9F => self.oam.read(address),
            // Unusable
            0xFEA0..=0xFEFF => 0xFF,
            // IO Registers
            0xFF00..=0xFF7F => self.io_registers.read(address),
            // HRAM
            0xFF80..=0xFFFE => self.hram.read(address),
            // Interrupt Enable Register
            0xFFFF => self.ie.read(),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // ROM
            0x0000..=0x7FFF => self.write_rom(address, value),
            // VRAM
            0x8000..=0x9FFF => self.vram.write(address, value),
            // External RAM
            0xA000..=0xBFFF => self.cartridge.write(address, value),
            // WRAM
            0xC000..=0xDFFF => self.wram.write(address, value),
            // Echo RAM
            0xE000..=0xFDFF => self.wram.write(address, value),
            // OAM
            0xFE00..=0xFE9F => self.oam.write(address, value),
            // Unusable
            0xFEA0..=0xFEFF => {}
            // IO Registers
            0xFF00..=0xFF7F => self.io_registers.write(address, value),
            // HRAM
            0xFF80..=0xFFFE => self.hram.write(address, value),
            // Interrupt Enable Register
            0xFFFF => self.ie.write(value),
        }
    }

    pub fn request_interrupt(&mut self, interrupt: InterruptType) {
        self.io_registers.request_interrupt(interrupt);
    }

    pub fn clear_interrupt(&mut self, interrupt: InterruptType) {
        self.io_registers.clear_interrupt(interrupt);
    }

    pub fn set_vram_accessibility(&mut self, accessible: bool) {
        self.vram.set_accessible(accessible);
    }

    pub fn set_oam_accessibility(&mut self, accessible: bool) {
        self.oam.set_accessible(accessible);
    }

    pub fn get_priority_interrupt(&self) -> Option<InterruptType> {
        let ei = self.read(InterruptType::IE_ADDRESS);
        let fi = self.read(InterruptType::IF_ADDRESS);
        let interrupts = ei & fi;
        for interrupt in 0..=4 {
            if interrupts & (1 << interrupt) != 0 {
                return Some(InterruptType::from_u8(interrupt));
            }
        }
        None
    }

    pub fn booting(&self) -> bool {
        self.io_registers.read(0xFF50) == 0
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.cartridge = Cartridge::new(data);
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        if self.booting() {
            // On the game boy color, the boot ROM is split into two parts:
            // 0x0000..=0x00FF and 0x0200..=0x08FF
            if let 0x0000..=0x00FF | 0x0200..=0x08FF = address {
                return self.boot_rom.read(address as usize);
            }
        }
        self.cartridge.read(address)
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        self.cartridge.write(address, value);
    }

    pub(crate) fn vram_mut(&mut self) -> &mut Vram {
        &mut self.vram
    }

    pub(crate) fn wram_mut(&mut self) -> &mut Wram {
        &mut self.wram
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for Bus {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}
