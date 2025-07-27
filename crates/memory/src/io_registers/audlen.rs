use crate::{AudioChannel, Bus, IOType};

pub trait AudLen {
    fn timer_length(&self) -> u8;
    fn set_timer_length(&mut self, value: u8);
    fn value(&self) -> u8;
    fn channel(&self) -> AudioChannel;
    fn io_register(&self) -> IOType;
}

pub enum WaveDuty {
    Duty12_5 = 0b00,
    Duty25 = 0b01,
    Duty50 = 0b10,
    Duty75 = 0b11,
}

impl WaveDuty {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0b00 => Self::Duty12_5,
            0b01 => Self::Duty25,
            0b10 => Self::Duty50,
            0b11 => Self::Duty75,
            _ => panic!("Invalid wave duty value: {}", value),
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn is_high(&self, step: u8) -> bool {
        self.pattern()[step as usize]
    }

    pub fn pattern(&self) -> [bool; 8] {
        match self {
            Self::Duty12_5 => [false, false, false, false, false, false, false, true],
            Self::Duty25 => [true, false, false, false, false, false, false, true],
            Self::Duty50 => [true, false, false, false, false, true, true, true],
            Self::Duty75 => [false, true, true, true, true, true, true, false],
        }
    }
}

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

    pub fn timer_length(&self) -> u8 {
        self.value & 0x3F
    }

    pub fn wave_duty(&self) -> WaveDuty {
        let value = (self.value & 0xC0) >> 6;
        WaveDuty::from_u8(value)
    }

    pub fn set_timer_length(&mut self, value: u8) {
        self.value = (self.value & 0xC0) | (value & 0x3F);
    }

    pub fn value(&self) -> u8 {
        self.value
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

    pub fn timer_length(&self) -> u8 {
        self.0.timer_length()
    }

    pub fn wave_duty(&self) -> WaveDuty {
        self.0.wave_duty()
    }

    fn set_timer_length(&mut self, value: u8) {
        self.0.set_timer_length(value)
    }

    fn value(&self) -> u8 {
        self.0.value()
    }

    fn channel(&self) -> AudioChannel {
        AudioChannel::Ch1
    }

    fn io_register(&self) -> IOType {
        IOType::AUD1LEN
    }
}

impl AudLen for Aud1Len {
    fn timer_length(&self) -> u8 {
        self.timer_length()
    }

    fn set_timer_length(&mut self, value: u8) {
        self.set_timer_length(value)
    }

    fn value(&self) -> u8 {
        self.value()
    }

    fn channel(&self) -> AudioChannel {
        self.channel()
    }

    fn io_register(&self) -> IOType {
        self.io_register()
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

    pub fn timer_length(&self) -> u8 {
        self.0.timer_length()
    }

    pub fn wave_duty(&self) -> WaveDuty {
        self.0.wave_duty()
    }

    fn set_timer_length(&mut self, value: u8) {
        self.0.set_timer_length(value)
    }

    fn value(&self) -> u8 {
        self.0.value()
    }

    fn channel(&self) -> AudioChannel {
        AudioChannel::Ch2
    }

    fn io_register(&self) -> IOType {
        IOType::AUD2LEN
    }
}

impl AudLen for Aud2Len {
    fn timer_length(&self) -> u8 {
        self.timer_length()
    }

    fn set_timer_length(&mut self, value: u8) {
        self.set_timer_length(value)
    }

    fn value(&self) -> u8 {
        self.value()
    }

    fn channel(&self) -> AudioChannel {
        self.channel()
    }

    fn io_register(&self) -> IOType {
        self.io_register()
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

    pub fn timer_length(&self) -> u8 {
        self.value
    }

    fn set_timer_length(&mut self, value: u8) {
        self.value = value;
    }

    fn value(&self) -> u8 {
        self.value
    }

    fn channel(&self) -> AudioChannel {
        AudioChannel::Ch3
    }

    fn io_register(&self) -> IOType {
        IOType::AUD3LEN
    }
}

impl AudLen for Aud3Len {
    fn timer_length(&self) -> u8 {
        self.timer_length()
    }

    fn set_timer_length(&mut self, value: u8) {
        self.set_timer_length(value)
    }

    fn value(&self) -> u8 {
        self.value()
    }

    fn channel(&self) -> AudioChannel {
        self.channel()
    }

    fn io_register(&self) -> IOType {
        self.io_register()
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

    pub fn timer_length(&self) -> u8 {
        self.value & 0x3F
    }

    fn set_timer_length(&mut self, value: u8) {
        self.value = value & 0x3F;
    }

    fn value(&self) -> u8 {
        self.value
    }

    fn channel(&self) -> AudioChannel {
        AudioChannel::Ch4
    }

    fn io_register(&self) -> IOType {
        IOType::AUD4LEN
    }
}

impl AudLen for Aud4Len {
    fn timer_length(&self) -> u8 {
        self.timer_length()
    }

    fn set_timer_length(&mut self, value: u8) {
        self.set_timer_length(value)
    }

    fn value(&self) -> u8 {
        self.value()
    }

    fn channel(&self) -> AudioChannel {
        self.channel()
    }

    fn io_register(&self) -> IOType {
        self.io_register()
    }
}
