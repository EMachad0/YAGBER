use yagber_memory::Bus;

pub struct Tile {
    pub data: [u8; 16],
}

impl Tile {
    pub fn from_memory(bus: &mut Bus, address: u16) -> Self {
        let mut data = [0; 16];
        for i in 0..16 {
            data[i] = bus.read(address + i as u16);
        }
        Self::new(data)
    }

    pub fn new(data: [u8; 16]) -> Self {
        Self { data }
    }

    /// Receives the pixel coordinate in the tile.
    /// Returns the colour index of the pixel.
    pub fn get_pixel(&self, x: u8, y: u8) -> u8 {
        let byte0 = self.data[y as usize * 2];
        let byte1 = self.data[y as usize * 2 + 1];
        let bit0 = byte0 & (1 << (7 - x));
        let bit0 = if bit0 != 0 { 0b01 } else { 0b00 };
        let bit1 = byte1 & (1 << (7 - x));
        let bit1 = if bit1 != 0 { 0b10 } else { 0b00 };
        (bit0 | bit1) as u8
    }
}
