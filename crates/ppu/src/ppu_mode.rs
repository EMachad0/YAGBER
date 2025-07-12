#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum PpuMode {
    /// 0b00
    #[default]
    HBlank,
    /// 0b01
    VBlank,
    /// 0b10
    OamScan,
    /// 0b11
    PixelTransfer,
}

impl PpuMode {
    #[allow(dead_code)]
    pub fn from_u8(value: u8) -> Self {
        match value & 0b11 {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::OamScan,
            3 => PpuMode::PixelTransfer,
            _ => unreachable!("Invalid mode value: {}", value),
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            PpuMode::HBlank => 0,
            PpuMode::VBlank => 1,
            PpuMode::OamScan => 2,
            PpuMode::PixelTransfer => 3,
        }
    }
}
