use crate::emulator::Emulator;

pub trait Runner {
    fn new(emulator: Emulator) -> Self;
    fn run(&mut self);
}
