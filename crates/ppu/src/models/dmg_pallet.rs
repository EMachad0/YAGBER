pub struct DmgPallet {
    value: u8,
}

impl DmgPallet {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn colour_index(&self, index: u8) -> u8 {
        (self.value >> (index * 2)) & 0b11
    }
}
