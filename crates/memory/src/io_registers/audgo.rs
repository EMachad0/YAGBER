use crate::{Bus, IOType};

#[derive(Debug, Clone, Copy)]
pub struct AudHigh {
    value: u8,
}

impl AudHigh {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus, io_type: IOType) -> Self {
        if !matches!(
            io_type,
            IOType::AUD1HIGH | IOType::AUD2HIGH | IOType::AUD3HIGH
        ) {
            panic!("Invalid IO type for AudHigh: {:?}", io_type);
        }
        Self::new(bus.read(io_type.address()))
    }

    pub fn trigger_enabled(&self) -> bool {
        self.value & 0x80 != 0
    }

    pub fn length_enabled(&self) -> bool {
        self.value & 0x40 != 0
    }

    pub fn period_high(&self) -> u8 {
        self.value & 0x07
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud1High(AudHigh);

impl Aud1High {
    pub fn new(value: u8) -> Self {
        Self(AudHigh::new(value))
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self(AudHigh::from_bus(bus, IOType::AUD1HIGH))
    }

    pub fn trigger_enabled(&self) -> bool {
        self.0.trigger_enabled()
    }

    pub fn length_enabled(&self) -> bool {
        self.0.length_enabled()
    }

    pub fn period_high(&self) -> u8 {
        self.0.period_high()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud2High(AudHigh);

impl Aud2High {
    pub fn new(value: u8) -> Self {
        Self(AudHigh::new(value))
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self(AudHigh::from_bus(bus, IOType::AUD2HIGH))
    }

    pub fn trigger_enabled(&self) -> bool {
        self.0.trigger_enabled()
    }

    pub fn length_enabled(&self) -> bool {
        self.0.length_enabled()
    }

    pub fn period_high(&self) -> u8 {
        self.0.period_high()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud3High(AudHigh);

impl Aud3High {
    pub fn new(value: u8) -> Self {
        Self(AudHigh::new(value))
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self(AudHigh::from_bus(bus, IOType::AUD3HIGH))
    }

    pub fn trigger_enabled(&self) -> bool {
        self.0.trigger_enabled()
    }

    pub fn length_enabled(&self) -> bool {
        self.0.length_enabled()
    }

    pub fn period_high(&self) -> u8 {
        self.0.period_high()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud4Go {
    value: u8,
}

impl Aud4Go {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self::new(bus.read(IOType::AUD4GO.address()))
    }

    pub fn trigger_enabled(&self) -> bool {
        self.value & 0x80 != 0
    }

    pub fn length_enabled(&self) -> bool {
        self.value & 0x40 != 0
    }
}
