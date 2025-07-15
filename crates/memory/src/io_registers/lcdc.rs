use crate::{Bus, IOType};

#[derive(Debug, Clone, Copy)]
pub enum TileSize {
    TileSize8,
    TileSize16,
}

impl TileSize {
    pub fn as_u8(&self) -> u8 {
        match self {
            TileSize::TileSize8 => 8,
            TileSize::TileSize16 => 16,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TileMapArea {
    /// 0x9800 - 0x9BFF
    TileMapArea0,
    /// 0x9C00 - 0x9FFF
    TileMapArea1,
}

#[derive(Debug, Clone, Copy)]
pub enum TileFetcherMode {
    TileDataArea0,
    TileDataArea1,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LcdcRegister {
    value: u8,
}

impl LcdcRegister {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            value: bus.read(IOType::LCDC.address()),
        }
    }

    pub fn lcd_ppu_enabled(&self) -> bool {
        self.value & 0x80 != 0
    }

    pub fn window_tile_map_area(&self) -> TileMapArea {
        if self.value & 0x40 != 0 {
            TileMapArea::TileMapArea1
        } else {
            TileMapArea::TileMapArea0
        }
    }

    pub fn lcd_window_enabled(&self) -> bool {
        self.value & 0x20 != 0
    }

    pub fn tile_data_area(&self) -> TileFetcherMode {
        if self.value & 0x10 != 0 {
            TileFetcherMode::TileDataArea1
        } else {
            TileFetcherMode::TileDataArea0
        }
    }

    pub fn bg_tile_map_area(&self) -> TileMapArea {
        if self.value & 0x08 != 0 {
            TileMapArea::TileMapArea1
        } else {
            TileMapArea::TileMapArea0
        }
    }

    pub fn obj_size(&self) -> TileSize {
        if self.value & 0x04 == 0 {
            TileSize::TileSize8
        } else {
            TileSize::TileSize16
        }
    }

    pub fn obj_enabled(&self) -> bool {
        self.value & 0x02 != 0
    }

    pub fn bg_window_enabled_priority(&self) -> bool {
        self.value & 0x01 != 0
    }
}
