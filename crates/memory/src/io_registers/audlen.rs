use crate::{Bus, IOType};

#[derive(Debug, Clone, Copy)]
struct PulseAudLen {
    value: u8,
}

impl PulseAudLen {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus, io_type: IOType) -> Self {
        if !matches!(io_type, IOType::AUD1LEN | IOType::AUD2LEN) {
            panic!("Invalid IO type for PulseAudLen: {:?}", io_type);
        }

        Self::new(bus.read(io_type.address()))
    }

    pub fn initial_timer_length(&self) -> u8 {
        self.value & 0x3F
    }

    pub fn wave_duty(&self) -> u8 {
        self.value & 0xC0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud1Len(PulseAudLen);

impl Aud1Len {
    pub fn new(value: u8) -> Self {
        Self(PulseAudLen::new(value))
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self(PulseAudLen::from_bus(bus, IOType::AUD1LEN))
    }

    pub fn initial_timer_length(&self) -> u8 {
        self.0.initial_timer_length()
    }

    pub fn wave_duty(&self) -> u8 {
        self.0.wave_duty()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud2Len(PulseAudLen);

impl Aud2Len {
    pub fn new(value: u8) -> Self {
        Self(PulseAudLen::new(value))
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self(PulseAudLen::from_bus(bus, IOType::AUD2LEN))
    }

    pub fn initial_timer_length(&self) -> u8 {
        self.0.initial_timer_length()
    }

    pub fn wave_duty(&self) -> u8 {
        self.0.wave_duty()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud3Len {
    value: u8,
}

impl Aud3Len {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self::new(bus.read(IOType::AUD3LEN.address()))
    }

    pub fn initial_timer_length(&self) -> u8 {
        self.value
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aud4Len {
    value: u8,
}

impl Aud4Len {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self::new(bus.read(IOType::AUD4LEN.address()))
    }

    pub fn initial_timer_length(&self) -> u8 {
        self.value & 0x3F
    }
}
