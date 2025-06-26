#[derive(Debug, Clone, Copy, Default)]
pub struct LcdcRegister {
    value: u8,
}

impl LcdcRegister {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn lcd_ppu_enabled(&self) -> bool {
        self.value & 0x80 != 0
    }

    // fn tile_map_area(&self, value: bool) -> (u16, u16) {
    //     if !value {
    //         (0x9800, 0x9BFF)
    //     } else {
    //         (0x9C00, 0x9FFF)
    //     }
    // }

    pub fn window_tile_map_area(&self) -> bool {
        self.value & 0x40 != 0
    }

    pub fn lcd_window_enabled(&self) -> bool {
        self.value & 0x20 != 0
    }

    pub fn tile_data_area(&self) -> bool {
        self.value & 0x10 != 0
    }

    pub fn bg_tile_map_area(&self) -> bool {
        self.value & 0x08 != 0
    }

    pub fn obj_size(&self) -> (u8, u8) {
        if self.value & 0x04 == 0 {
            (8, 8)
        } else {
            (8, 16)
        }
    }

    pub fn obj_enabled(&self) -> bool {
        self.value & 0x02 != 0
    }

    pub fn bg_window_enabled_priority(&self) -> bool {
        self.value & 0x01 != 0
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn write(&mut self, value: u8) {
        self.value = value;
    }
}

impl crate::Register for LcdcRegister {
    fn read(&self) -> u8 {
        self.read()
    }

    fn write(&mut self, value: u8) {
        self.write(value);
    }
}
