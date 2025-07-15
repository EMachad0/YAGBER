use crate::models::attribute::TileAttr;

#[derive(Debug, Clone, Copy)]
pub struct Object {
    x: u8,
    y: u8,
    tile_index: u8,
    attr: TileAttr,
}

impl Object {
    pub fn new(x: u8, y: u8, tile_index: u8, attr: u8) -> Self {
        let attr = TileAttr::new(attr);
        Self {
            x,
            y,
            tile_index,
            attr,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::new(bytes[1], bytes[0], bytes[2], bytes[3])
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn tile_index_8(&self) -> u8 {
        self.tile_index
    }

    pub fn tile_index_16(&self) -> (u8, u8) {
        let index = self.tile_index;
        let upper = index & 0xFE;
        let lower = index | 0x01;
        if self.attr().y_flip() {
            (lower, upper)
        } else {
            (upper, lower)
        }
    }

    pub fn attr(&self) -> &TileAttr {
        &self.attr
    }
}
