use crate::Bus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedButtons {
    Both = 0x00,
    Buttons = 0x10,
    Directions = 0x20,
    None = 0x30,
}

impl SelectedButtons {
    pub fn as_bits(&self) -> u8 {
        *self as u8
    }

    pub fn from_bits(bits: u8) -> Self {
        match bits {
            0x00 => SelectedButtons::Both,
            0x10 => SelectedButtons::Buttons,
            0x20 => SelectedButtons::Directions,
            0x30 => SelectedButtons::None,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JoypRegister {
    value: u8,
}

impl JoypRegister {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_memory(bus: &mut Bus) -> Self {
        Self {
            value: bus.read(0xFF00),
        }
    }

    pub fn selected_buttons(&self) -> SelectedButtons {
        SelectedButtons::from_bits(self.value & 0x30)
    }
}
