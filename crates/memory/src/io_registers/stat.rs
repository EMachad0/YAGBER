use yagber_app::{EdgeDetector, EdgeMode};

use crate::{IOType, InterruptType, Register};

#[derive(Debug, Clone, Copy)]
pub struct StatRegister {
    value: u8,
    edge_detector: EdgeDetector,
}

impl StatRegister {
    pub fn new() -> Self {
        Self {
            value: 0,
            edge_detector: EdgeDetector::new(EdgeMode::Rising),
        }
    }

    fn lyc_eq_ly_select(&self) -> bool {
        self.value & 0x40 != 0
    }

    fn mode_2_select(&self) -> bool {
        self.value & 0x20 != 0
    }

    fn mode_1_select(&self) -> bool {
        self.value & 0x10 != 0
    }

    fn mode_0_select(&self) -> bool {
        self.value & 0x08 != 0
    }

    fn lyc_eq_ly(&self) -> bool {
        self.value & 0x04 != 0
    }

    pub fn set_lyc_eq_ly(&mut self, value: bool) {
        if value {
            self.value |= 0x04;
        } else {
            self.value &= !0x04;
        }
    }

    fn mode(&self) -> u8 {
        self.value & 0x03
    }

    pub fn set_mode(&mut self, mode: u8) {
        self.value = (self.value & 0xFC) | (mode & 0x03);
    }

    fn interrupt_line(&self) -> bool {
        (self.lyc_eq_ly_select() && self.lyc_eq_ly())
            || (self.mode_0_select() && self.mode() == 0)
            || (self.mode_1_select() && self.mode() == 1)
            || (self.mode_2_select() && self.mode() == 2)
    }

    pub fn should_trigger_interrupt(&mut self) -> bool {
        self.edge_detector.tick(self.interrupt_line())
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn write(&mut self, value: u8) {
        self.value = (self.value & 0x07) | (value & !0x07);
    }

    pub fn on_dot_cycle(emulator: &mut yagber_app::Emulator, _event: &yagber_app::DotCycleEvent) {
        let _span = tracing::info_span!("stat dot cycle").entered();
        let bus = emulator.get_component_mut::<crate::Bus>().unwrap();
        let ly = bus.read(IOType::LY.address());
        let lyc = bus.read(IOType::LYC.address());

        let stat = bus
            .io_registers
            .get_register_mut::<StatRegister>(IOType::STAT)
            .expect("STAT register not found");
        stat.set_lyc_eq_ly(lyc == ly);

        if stat.should_trigger_interrupt() {
            bus.request_interrupt(InterruptType::Lcd);
        }
    }
}

impl Register for StatRegister {
    fn read(&self) -> u8 {
        self.read()
    }

    fn write(&mut self, value: u8) {
        self.write(value);
    }
}

impl Default for StatRegister {
    fn default() -> Self {
        Self::new()
    }
}
