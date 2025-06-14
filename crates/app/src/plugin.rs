use crate::Emulator;

pub trait Plugin {
    fn init(self, emulator: &mut Emulator);
}
