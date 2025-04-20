use crate::memory::Memory;

#[derive(Debug, Default, Clone)]
pub struct Ram {
    offset: u16,
    data: Box<[Option<u8>]>,
}

impl Ram {
    pub fn new(size: u16, offset: u16) -> Self {
        let data = vec![None; size as usize].into_boxed_slice();
        Self { offset, data }
    }

    pub fn read(&self, address: u16) -> u8 {
        if cfg!(feature = "break_on_unitialized_ram_read") && self.data[address as usize].is_none()
        {
            panic!("Uninitialized RAM read at address: {:#X}", address);
        }
        let address = address.wrapping_sub(self.offset) as usize;
        self.data[address].unwrap_or(0xFF)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let address = address.wrapping_sub(self.offset) as usize;
        self.data[address] = Some(value);
    }
}

impl Memory for Ram {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}
