use crate::{
    io_registers::io_type::IOType,
    memory::Memory,
    register::{ByteRegister, Register},
};

pub const IO_REGISTERS_OFFSET: u16 = 0xFF00;
pub const IO_REGISTERS_SIZE: usize = 0x0080;

pub struct IOBus {
    data: Vec<Box<dyn Register>>,
}

impl IOBus {
    pub fn new() -> Self {
        let data = (0..IO_REGISTERS_SIZE)
            .map(|_| Box::new(ByteRegister::new(0x00)) as Box<dyn Register>)
            .collect::<Vec<_>>();

        Self { data }
    }

    pub fn with_register<R: Register + 'static>(mut self, io_type: IOType, register: R) -> Self {
        let address = io_type.address() - IO_REGISTERS_OFFSET;
        self.data[address as usize] = Box::new(register);
        self
    }

    pub fn read(&self, address: u16) -> u8 {
        let address = address.wrapping_sub(IO_REGISTERS_OFFSET);
        self.data[address as usize].read()
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let address = address.wrapping_sub(IO_REGISTERS_OFFSET);
        self.data[address as usize].write(value);
    }
}

impl Default for IOBus {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for IOBus {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}

impl std::fmt::Debug for IOBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IOBus").finish()
    }
}
