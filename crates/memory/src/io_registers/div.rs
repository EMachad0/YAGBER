use crate::{Bus, IOType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacClock {
    Every256M,
    Every4M,
    Every16M,
    Every64M,
}

impl TacClock {
    /// Returns the bit that is used to determine the frequency of the timer.
    /// Cycle frequency is determined by the bit times two due to using a falling edge detector.
    pub fn div_bit(&self) -> u8 {
        match self {
            Self::Every256M => 7, // Every 256 M-Cycles
            Self::Every4M => 1,   // Every 4 M-Cycles
            Self::Every16M => 3,  // Every 16 M-Cycles
            Self::Every64M => 5,  // Every 64 M-Cycles
        }
    }

    /// Returns the bit mask that is used to determine the frequency of the timer.
    /// Cycle frequency is determined by the bit times two due to using a falling edge detector.
    pub fn div_mask(&self) -> u16 {
        1 << self.div_bit()
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0b00 => Self::Every256M,
            0b01 => Self::Every4M,
            0b10 => Self::Every16M,
            0b11 => Self::Every64M,
            _ => unreachable!("Invalid TAC mode: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DivRegister;

#[derive(Debug, Clone, Copy)]
pub struct TacRegister {
    value: u8,
}

impl TacRegister {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            value: bus.read(IOType::TAC.address()),
        }
    }

    pub fn enabled(&self) -> bool {
        self.value & 0x04 != 0
    }

    pub fn clock_select(&self) -> TacClock {
        TacClock::from_u8(self.value & 0x03)
    }
}
