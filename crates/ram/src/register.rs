#[derive(Debug, Default, Clone, Copy)]
pub struct Register {
    value: u8,
}

impl Register {
    pub fn new(value: u8) -> Self {
        Register { value }
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn write(&mut self, value: u8) {
        self.value = value;
    }
}
