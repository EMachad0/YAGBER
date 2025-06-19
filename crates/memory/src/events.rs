use yagber_app::Event;

#[derive(Debug)]
pub struct MemoryWriteEvent {
    pub address: u16,
    pub value: u8,
}

impl MemoryWriteEvent {
    pub fn new(address: u16, value: u8) -> Self {
        Self { address, value }
    }
}

impl Event for MemoryWriteEvent {}
