use crate::events::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmulatorState {
    Init,
    Running,
    Paused,
    Stopped,
    Ending,
    Ended,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateTransitionEvent {
    pub from: EmulatorState,
    pub to: EmulatorState,
}

impl Event for StateTransitionEvent {}
