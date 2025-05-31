use crate::{InterruptType, memory::Memory};

pub const IO_REGISTERS_OFFSET: u16 = 0xFF00;
pub const IO_REGISTERS_SIZE: usize = 0x007F;

#[derive(Debug, Clone, Copy)]
pub struct IORegisters {
    data: [u8; IO_REGISTERS_SIZE],
}

impl IORegisters {
    pub fn new() -> Self {
        Self {
            data: [0; IO_REGISTERS_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let address = address.wrapping_sub(IO_REGISTERS_OFFSET);
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let address = address.wrapping_sub(IO_REGISTERS_OFFSET);
        self.data[address as usize] = value;
    }

    pub fn request_interrupt(&mut self, interrupt: InterruptType) {
        self.set_bit(InterruptType::IF_ADDRESS, interrupt.to_u8());
    }

    pub fn clear_interrupt(&mut self, interrupt: InterruptType) {
        self.clear_bit(InterruptType::IF_ADDRESS, interrupt.to_u8());
    }
}

impl Default for IORegisters {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for IORegisters {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}
