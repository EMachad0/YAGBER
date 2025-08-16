use crate::cartridges::RtcRegisterKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalRamAddress {
    /// Address for the external RAM in the cartridge.
    ExternalRam(usize),
    /// Address for the RTC in the cartridge.
    Rtc(RtcRegisterKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MbcDeviceUpdate {
    RtcLatch,
    RumbleMotor(bool),
}

