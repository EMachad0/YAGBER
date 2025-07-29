use crate::{
    ByteRegister, IOType, InterruptType, boot_rom::BootRom, cartridge::Cartridge, cram::Cram,
    io_registers::IOBus, memory::Memory, oam::Oam, ram::Ram, save::NativeFileBackend, vram::Vram,
    wram::Wram,
};

#[derive(Debug)]
pub struct Bus {
    boot_rom: BootRom,
    cartridge: Cartridge<NativeFileBackend>,
    pub io_registers: IOBus,
    hram: Ram,
    ie: ByteRegister,
    pub vram: Vram,
    pub wram: Wram,
    pub oam: Oam,
    pub background_cram: Cram,
    pub object_cram: Cram,
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
            ie: ByteRegister::new(0x00),
            background_cram: Cram::new(),
            object_cram: Cram::new(),
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
            0xE000..=0xFDFF => self.wram.write(address - 0x2000, value),
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
        let bit = 1 << interrupt.bit();
        let if_reg = self.read(IOType::IF.address());
        let new_if_reg = if_reg | bit;
        if new_if_reg != if_reg {
            self.io_registers.write(IOType::IF.address(), new_if_reg);
        }
    }

    pub fn clear_interrupt(&mut self, interrupt: InterruptType) {
        let bit = 1 << interrupt.bit();
        let if_reg = self.read(IOType::IF.address());
        let new_if_reg = if_reg & !bit;
        if new_if_reg != if_reg {
            self.io_registers.write(IOType::IF.address(), new_if_reg);
        }
    }

    pub fn get_priority_interrupt(&self) -> Option<InterruptType> {
        let ei = self.read(IOType::IE.address());
        let fi = self.read(IOType::IF.address());
        let interrupts = ei & fi;
        for interrupt in 0..=4 {
            if interrupts & (1 << interrupt) != 0 {
                return Some(InterruptType::from_u8(interrupt));
            }
        }
        None
    }

    pub fn booting(&self) -> bool {
        self.io_registers.read(IOType::BANK.address()) == 0
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.cartridge = Cartridge::new(data);
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        if self.booting() {
            // On the game boy colour, the boot ROM is split into two parts:
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

impl yagber_app::Component for Bus {}
