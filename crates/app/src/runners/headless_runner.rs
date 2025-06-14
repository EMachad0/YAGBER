use crate::{emulator::Emulator, runners::Runner};

#[derive(Debug)]
pub struct HeadlessRunner {
    emulator: Emulator,
}

impl Runner for HeadlessRunner {
    type Result = ();

    fn new(emulator: Emulator) -> Self {
        Self { emulator }
    }

    fn run(mut self) {
        // Never exit
        loop {
            self.emulator.step();
        }
    }
}
