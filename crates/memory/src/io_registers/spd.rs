use yagber_app::SpeedMode;

use crate::{Bus, IOType};

#[derive(Debug, Clone, Copy)]
pub struct Spd {
    value: u8,
}

impl Spd {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self::new(bus.read(IOType::SPD.address()))
    }

    pub fn speed_mode(&self) -> SpeedMode {
        if self.value & 0x80 == 0 {
            SpeedMode::Single
        } else {
            SpeedMode::Double
        }
    }

    pub fn speed_switch_armed(&self) -> bool {
        (self.value & 0x01) != 0
    }

    pub(crate) fn spd_transformer((_old_value, new_value): (u8, u8)) -> Option<u8> {
        Some(new_value & 0x01)
    }

    pub(crate) fn on_spd_write(emulator: &mut yagber_app::Emulator, value: u8) {
        let spd = Spd::new(value);
        let speed_mode = spd.speed_mode();
        emulator.set_speed_mode(speed_mode);
    }

    pub(crate) fn emu_spd_hook(emulator: &mut yagber_app::Emulator) -> impl Fn(u8) + use<> {
        let emulator_ptr = emulator as *mut yagber_app::Emulator;
        move |value| Spd::on_spd_write(unsafe { &mut *emulator_ptr }, value)
    }
}
