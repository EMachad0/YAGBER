use crate::register::Register;

#[derive(Default)]
pub struct UnconnectedRegister;

impl Register for UnconnectedRegister {
    fn read(&self) -> u8 {
        0xFF
    }

    fn write(&mut self, _value: u8) {}
}
