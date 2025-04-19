#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// 0b00
    HBlank,
    /// 0b01
    VBlank,
    /// 0b10
    OamScan,
    /// 0b11
    PixelTransfer,
}

impl Mode {
    /// Duration of each mode in dots.
    pub fn duration(&self) -> u32 {
        match self {
            Mode::HBlank => 204,
            Mode::VBlank => 4560,
            Mode::OamScan => 80,
            Mode::PixelTransfer => 172,
        }
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::OamScan,
            3 => Mode::PixelTransfer,
            _ => panic!("Invalid mode value"),
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Mode::HBlank => 0,
            Mode::VBlank => 1,
            Mode::OamScan => 2,
            Mode::PixelTransfer => 3,
        }
    }
}
