#[derive(Debug, Clone, Copy)]
pub enum VramBank {
    Bank0,
    Bank1,
}

impl VramBank {
    pub fn value(&self) -> bool {
        match self {
            VramBank::Bank0 => false,
            VramBank::Bank1 => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PaletteIndex {
    value: u8,
}

impl PaletteIndex {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileAttr {
    value: u8,
}

impl TileAttr {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn priority(&self) -> bool {
        self.value & 0x80 != 0
    }

    pub fn y_flip(&self) -> bool {
        self.value & 0x40 != 0
    }

    pub fn x_flip(&self) -> bool {
        self.value & 0x20 != 0
    }

    pub fn dmg_palette(&self) -> PaletteIndex {
        PaletteIndex::new((self.value >> 4) & 1)
    }

    pub fn vram_bank(&self) -> VramBank {
        if self.value & 0x08 != 0 {
            VramBank::Bank1
        } else {
            VramBank::Bank0
        }
    }

    pub fn cgb_palette(&self) -> PaletteIndex {
        PaletteIndex::new(self.value & 0x07)
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}
