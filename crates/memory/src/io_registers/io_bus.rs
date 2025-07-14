use crate::{IOType, io_registers::io_register::IORegister, memory::Memory};

pub struct IOBus {
    data: Vec<IORegister>,
}

impl IOBus {
    pub const IO_REGISTERS_OFFSET: u16 = 0xFF00;
    pub const IO_REGISTERS_SIZE: usize = 0x0080;

    pub fn new() -> Self {
        let data = (0..Self::IO_REGISTERS_SIZE)
            .map(|_| IORegister::new())
            .collect::<Vec<_>>();

        Self { data }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[Self::virtual_address(address)].read()
    }

    pub fn write(&mut self, address: u16, value: u8) {
        #[cfg(feature = "trace")]
        tracing::trace!(
            "IOBus write: {:?} {:#04x} = {:#04x}",
            IOType::from_address(address),
            address,
            value
        );
        self.data[Self::virtual_address(address)].write(value);
    }

    pub fn write_unchecked(&mut self, address: u16, value: u8) {
        #[cfg(feature = "trace")]
        tracing::trace!(
            "IOBus write_unchecked: {:?} {:#04x} = {:#04x}",
            IOType::from_address(address),
            address,
            value
        );
        self.data[Self::virtual_address(address)].write_unchecked(value);
    }

    pub fn write_unhooked(&mut self, address: u16, value: u8) {
        #[cfg(feature = "trace")]
        tracing::trace!(
            "IOBus write_unhooked: {:?} {:#04x} = {:#04x}",
            IOType::from_address(address),
            address,
            value
        );
        self.data[Self::virtual_address(address)].write_unhooked(value);
    }

    pub fn add_transformer<F>(&mut self, io: IOType, transformer: F) -> &mut Self
    where
        F: Fn(u8, u8) -> Option<u8> + 'static,
    {
        self.data[Self::virtual_address(io.address())].add_transformer(transformer);
        self
    }

    pub fn add_hook<F>(&mut self, io: IOType, hook: F)
    where
        F: Fn(u8) + 'static,
    {
        self.data[Self::virtual_address(io.address())].add_hook(hook);
    }

    pub fn add_reader<F>(&mut self, io: IOType, reader: F) -> &mut Self
    where
        F: Fn(u8) -> u8 + 'static,
    {
        self.data[Self::virtual_address(io.address())].add_reader(reader);
        self
    }

    pub fn with_transformer<F>(&mut self, io: IOType, transformer: F) -> &mut Self
    where
        F: Fn(u8, u8) -> Option<u8> + 'static,
    {
        self.add_transformer(io, transformer);
        self
    }

    pub fn with_hook<F>(&mut self, io: IOType, hook: F) -> &mut Self
    where
        F: Fn(u8) + 'static,
    {
        self.add_hook(io, hook);
        self
    }

    pub fn with_reader<F>(&mut self, io: IOType, reader: F) -> &mut Self
    where
        F: Fn(u8) -> u8 + 'static,
    {
        self.add_reader(io, reader);
        self
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
