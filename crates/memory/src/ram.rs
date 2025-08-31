use crate::memory::Memory;

#[derive(Debug, Default, Clone)]
pub struct Ram {
    offset: usize,
    data: Box<[Option<u8>]>,
}

impl Ram {
    pub fn new(size: usize, offset: usize) -> Self {
        let data = vec![None; size].into_boxed_slice();
        Self { offset, data }
    }

    pub fn from_bytes(data: &[u8], offset: usize) -> Self {
        let data = data
            .iter()
            .map(|&byte| Some(byte))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { offset, data }
    }

    pub fn read_usize(&self, address: usize) -> u8 {
        let uaddress = address.wrapping_sub(self.offset);
        if cfg!(feature = "warn_on_unitialized_ram_read") && self.data[uaddress].is_none() {
            #[cfg(feature = "trace")]
            tracing::warn!("Uninitialized RAM read at address: {address:#X}");
        }
        self.data[uaddress].unwrap_or(0xFF)
    }

    pub fn write_usize(&mut self, address: usize, value: u8) {
        let address = address.wrapping_sub(self.offset);
        self.data[address] = Some(value);
    }

    pub(crate) fn data_slice(&self) -> &[Option<u8>] {
        &self.data
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data
            .to_vec()
            .iter()
            .map(|v| v.unwrap_or_default())
            .collect()
    }
}

impl Memory for Ram {
    fn read(&self, address: u16) -> u8 {
        self.read_usize(address as usize)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write_usize(address as usize, value);
    }
}
