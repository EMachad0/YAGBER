use crate::emulator::Emulator;

pub trait Runner {
    type Result;

    fn new(emulator: Emulator) -> Self;
    fn run(self) -> Self::Result;
}
