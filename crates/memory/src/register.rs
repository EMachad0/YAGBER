pub trait Register: yagber_app::Downcastable + 'static {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8);
}

/// A register that can be read and written to as a single byte.
#[derive(Debug, Copy, Clone)]
pub struct ByteRegister {
    data: u8,
}

impl ByteRegister {
    pub fn new(data: u8) -> Self {
        Self { data }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn write(&mut self, value: u8) {
        self.data = value;
    }
}

impl Register for ByteRegister {
    fn read(&self) -> u8 {
        self.read()
    }

    fn write(&mut self, value: u8) {
        self.write(value);
    }
}
