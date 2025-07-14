#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum InterruptType {
    VBlank = 0,
    Lcd = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

impl InterruptType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::VBlank,
            1 => Self::Lcd,
            2 => Self::Timer,
            3 => Self::Serial,
            4 => Self::Joypad,
            _ => panic!("Invalid interrupt type"),
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn address(self) -> u16 {
        match self {
            Self::VBlank => 0x0040,
            Self::Lcd => 0x0048,
            Self::Timer => 0x0050,
            Self::Serial => 0x0058,
            Self::Joypad => 0x0060,
        }
    }
}
