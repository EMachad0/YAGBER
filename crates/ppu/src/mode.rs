#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// 0b00
    HBlank,
    /// 0b01
    VBlank,
    /// 0b10
    OamScam,
    /// 0b11
    PixelTransfer,
}

impl Mode {
    pub fn duration(&self) -> u32 {
        match self {
            Mode::HBlank => 204,
            Mode::VBlank => 4560,
            Mode::OamScam => 80,
            Mode::PixelTransfer => 172,
        }
    }
}
