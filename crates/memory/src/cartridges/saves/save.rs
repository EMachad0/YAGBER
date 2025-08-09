use crate::cartridges::rtc::RtcRegisters;

/// Save data structure for a cartridge.
///
/// Default is used for first time initialization.
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Save {
    /// Ram data of the save.
    pub data: Option<Vec<u8>>,
    /// Real time clock
    pub rtc_registers: Option<RtcRegisters>,
    /// Timestanp of the save in seconds since the Unix epoch.
    pub timestamp: i64,
}
