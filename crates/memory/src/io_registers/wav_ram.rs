use crate::{Bus, IOType};

#[derive(Debug, Clone)]
pub struct WavRam {
    values: [u8; 16],
}

impl WavRam {
    pub fn new(values: [u8; 16]) -> Self {
        Self { values }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        let st = IOType::WAV.address();
        let values = [
            bus.read(st + 0),
            bus.read(st + 1),
            bus.read(st + 2),
            bus.read(st + 3),
            bus.read(st + 4),
            bus.read(st + 5),
            bus.read(st + 6),
            bus.read(st + 7),
            bus.read(st + 8),
            bus.read(st + 9),
            bus.read(st + 10),
            bus.read(st + 11),
            bus.read(st + 12),
            bus.read(st + 13),
            bus.read(st + 14),
            bus.read(st + 15),
        ];
        Self { values }
    }

    pub fn read(&self, index: u8) -> u8 {
        let inner = (index / 2) as usize;
        let nibble = (index % 2) as usize;
        let value = self.values[inner];
        if nibble == 0 {
            (value & 0xF0) >> 4
        } else {
            value & 0x0F
        }
    }
}
