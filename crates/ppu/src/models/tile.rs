use crate::models::attribute::TileAttr;
use yagber_memory::Bus;

pub struct Tile {
    pub data: [u8; 16],
    pub attr: TileAttr,
}

impl Tile {
    pub fn from_memory(bus: &Bus, address: u16, attr: u8) -> Self {
        let attr = TileAttr::new(attr);

        let mut data = [0; 16];
        let tile_data = bus.vram.tile(attr.vram_bank().value(), address);
        for (i, data) in data.iter_mut().enumerate() {
            *data = tile_data[i].unwrap();
        }
        Self::new(data, attr)
    }

    pub fn new(data: [u8; 16], attr: TileAttr) -> Self {
        Self { data, attr }
    }

    /// Receives the pixel coordinate in the tile.
    /// Returns the colour index of the pixel.
    pub fn colour_index(&self, mut x: u8, mut y: u8) -> u8 {
        if self.attr.x_flip() {
            x = 7 - x;
        }
        if self.attr.y_flip() {
            y = 7 - y;
        }
        let byte0 = self.data[y as usize * 2];
        let byte1 = self.data[y as usize * 2 + 1];
        let bit0 = byte0 & (1 << (7 - x));
        let bit0 = if bit0 != 0 { 0b01 } else { 0b00 };
        let bit1 = byte1 & (1 << (7 - x));
        let bit1 = if bit1 != 0 { 0b10 } else { 0b00 };
        (bit0 | bit1) as u8
    }

    pub fn get_pixel_row(&self, mut y: u8) -> [u8; 8] {
        if self.attr.y_flip() {
            y = 7 - y;
        }
        let byte0 = self.data[y as usize * 2];
        let byte1 = self.data[y as usize * 2 + 1];
        let mut row = [0; 8];
        for (x, val) in row.iter_mut().enumerate() {
            let bit0 = byte0 & (1 << (7 - x));
            let bit0 = if bit0 != 0 { 0b01 } else { 0b00 };
            let bit1 = byte1 & (1 << (7 - x));
            let bit1 = if bit1 != 0 { 0b10 } else { 0b00 };
            *val = bit0 | bit1;
        }
        if self.attr.x_flip() {
            row.reverse();
        }
        row
    }
}
