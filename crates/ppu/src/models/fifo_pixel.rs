use crate::models::PaletteIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FifoPixelType {
    Background,
    Object,
}

pub struct FifoPixel {
    colour_index: u8,
    palette_index: PaletteIndex,
    priority: bool,
    pixel_type: FifoPixelType,
}

impl FifoPixel {
    pub fn new(
        colour_index: u8,
        palette_index: PaletteIndex,
        priority: bool,
        pixel_type: FifoPixelType,
    ) -> Self {
        Self {
            colour_index,
            palette_index,
            priority,
            pixel_type,
        }
    }

    pub fn colour_index(&self) -> u8 {
        self.colour_index
    }

    pub fn palette_index(&self) -> PaletteIndex {
        self.palette_index
    }

    pub fn priority(&self) -> bool {
        self.priority
    }

    pub fn pixel_type(&self) -> FifoPixelType {
        self.pixel_type
    }
}
