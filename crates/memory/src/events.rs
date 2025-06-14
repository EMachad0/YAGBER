use yagber_app::Event;

#[derive(Debug)]
pub struct MemoryWriteEvent {
    pub address: u16,
    pub value: u8,
}

impl Event for MemoryWriteEvent {}
