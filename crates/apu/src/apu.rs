use crate::channels::Ch1;

pub struct Apu {
    ch1: Ch1,
}

impl Apu {
    pub fn new() -> Self {
        Self { ch1: Ch1 }
    }
}
