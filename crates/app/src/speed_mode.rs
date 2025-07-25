#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SpeedMode {
    #[default]
    Single,
    Double,
}

impl SpeedMode {
    pub fn toggle(&self) -> Self {
        match self {
            SpeedMode::Single => SpeedMode::Double,
            SpeedMode::Double => SpeedMode::Single,
        }
    }

    pub fn as_spd_bit(&self) -> u8 {
        match self {
            SpeedMode::Single => 0,
            SpeedMode::Double => 0x80,
        }
    }
}
