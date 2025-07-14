use crate::Bus;

#[derive(Default, Debug, Clone, Copy)]
pub struct CramSpecification {
    value: u8,
}

impl CramSpecification {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub(crate) fn value(&self) -> u8 {
        self.value
    }

    pub fn from_bus(bus: &Bus, address: u16) -> Self {
        Self {
            value: bus.read(address),
        }
    }

    pub fn index(&self) -> usize {
        (self.value & 0x3F) as usize
    }

    pub fn auto_increment(&self) -> bool {
        self.value & 0x80 != 0
    }
}

#[derive(Debug, Clone)]
pub struct Cram {
    data: [u8; Self::SIZE],
}

impl Cram {
    const SIZE: usize = 64;

    pub fn new() -> Self {
        Self {
            data: [0; Self::SIZE],
        }
    }

    pub fn read_data(&self, specification: &CramSpecification) -> u8 {
        self.data[specification.index()]
    }

    pub fn write_data(&mut self, specification: &CramSpecification, value: u8) {
        self.data[specification.index()] = value;
    }

    /// Reads a colour from the CRAM.
    /// Returns the RGB555 value of the colour.
    pub fn read_colour(&self, palette_index: u8, colour_index: u8) -> u16 {
        let palette_index = palette_index as usize;
        let colour_index = colour_index as usize;
        let offset = (palette_index * 4 + colour_index) * 2;
        let colour_lo = self.data[offset];
        let colour_hi = self.data[offset + 1];
        u16::from_le_bytes([colour_lo, colour_hi])
    }
}

impl Default for Cram {
    fn default() -> Self {
        Self::new()
    }
}
