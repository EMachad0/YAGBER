/// Save data structure for a cartridge.
///
/// Default is used for first time initialization.
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Save {
    /// Ram data of the save.
    pub data: Vec<u8>,
    /// Timestanp of the save in seconds since the Unix epoch.
    pub timestamp: i64,
}

