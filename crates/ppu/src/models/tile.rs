use yagber_memory::Bus;

pub struct TileAttr {
    value: u8,
}

impl TileAttr {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn palette_index(&self) -> u8 {
        self.value & 0x07
    }

    pub fn bank(&self) -> bool {
        self.value & 0x08 != 0
    }

    pub fn x_flip(&self) -> bool {
        self.value & 0x20 != 0
    }

    pub fn y_flip(&self) -> bool {
        self.value & 0x40 != 0
    }

    #[allow(dead_code)]
    pub fn bg_priority(&self) -> bool {
        self.value & 0x80 != 0
    }
}

pub struct Tile {
    pub data: [u8; 16],
    pub attr: TileAttr,
}

impl Tile {
    pub fn from_memory(bus: &Bus, address: u16, attr: u8) -> Self {
        let attr = TileAttr::new(attr);

        let mut data = [0; 16];
        let tile_data = bus.vram.tile(attr.bank(), address);
        for (i, data) in data.iter_mut().enumerate() {
            *data = tile_data[i].unwrap();
        }
        Self::new(data, attr)
    }

    pub fn new(data: [u8; 16], attr: TileAttr) -> Self {
        Self { data, attr }
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
