pub trait Memory {
    fn read(&self, address: u16) -> u8;

    fn read_u16(&self, address: u16) -> u16 {
        let lo = self.read(address);
        let hi = self.read(address + 1);
        u16::from_le_bytes([lo, hi])
    }

    fn read_masked(&self, address: u16, mask: u8) -> u8 {
        let value = self.read(address);
        value & mask
    }

    fn read_bit(&self, address: u16, bit: u8) -> bool {
        let value = self.read(address);
        (value & (1 << bit)) != 0
    }

    fn write(&mut self, address: u16, value: u8);

    fn write_masked(&mut self, address: u16, value: u8, mask: u8) {
        let current_value = self.read(address);
        let new_value = (current_value & !mask) | (value & mask);
        self.write(address, new_value);
    }

    fn write_u16(&mut self, address: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write(address, lo);
        self.write(address + 1, hi);
    }

    fn set_bit(&mut self, address: u16, bit: u8) {
        let value = self.read(address);
        let new_value = value | (1 << bit);
        self.write(address, new_value);
    }

    fn clear_bit(&mut self, address: u16, bit: u8) {
        let value = self.read(address);
        let new_value = value & !(1 << bit);
        self.write(address, new_value);
    }
}
