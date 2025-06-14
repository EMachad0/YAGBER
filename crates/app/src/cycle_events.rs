use crate::Event;

#[derive(Debug)]
pub struct MCycleEvent {
    pub cycle: u64,
}

impl Event for MCycleEvent {}

#[derive(Debug)]
pub struct TCycleEvent {
    pub cycle: u64,
}

impl Event for TCycleEvent {}

#[derive(Debug)]
pub struct DotCycleEvent {
    pub cycle: u64,
}

impl Event for DotCycleEvent {}
