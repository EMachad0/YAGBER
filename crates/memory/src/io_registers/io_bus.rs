use crate::{
    io_registers::{io_type::IOType, lcdc::LcdcRegister},
    memory::Memory,
    register::{ByteRegister, Register},
};

type RegisterBox = Box<dyn Register>;

pub struct IOBus {
    data: Vec<RegisterBox>,
}

impl IOBus {
    pub const IO_REGISTERS_OFFSET: u16 = 0xFF00;
    pub const IO_REGISTERS_SIZE: usize = 0x0080;

    pub fn new() -> Self {
        let data = (0..Self::IO_REGISTERS_SIZE)
            .map(|_| Box::new(ByteRegister::new(0x00)) as RegisterBox)
            .collect::<Vec<_>>();

        Self { data }.with_register(IOType::LCDC, LcdcRegister::new())
    }

    pub fn with_register<R: Register>(mut self, io_type: IOType, register: R) -> Self {
        self.data[Self::virtual_address(io_type.address())] = Box::new(register);
        self
    }

    pub fn get_register<R: Register>(&self, io_type: IOType) -> Option<&R> {
        self.data[Self::virtual_address(io_type.address())]
            .as_any_ref()
            .downcast_ref::<R>()
    }

    pub fn get_register_mut<R: Register>(&mut self, io_type: IOType) -> Option<&mut R> {
        self.data[Self::virtual_address(io_type.address())]
            .as_any_mut()
            .downcast_mut::<R>()
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[Self::virtual_address(address)].read()
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[Self::virtual_address(address)].write(value);
    }

    fn virtual_address(address: u16) -> usize {
        (address - Self::IO_REGISTERS_OFFSET) as usize
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
