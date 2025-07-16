use yagber_app::{EdgeDetector, EdgeMode};

use crate::{Bus, IOType, InterruptType};

pub struct Stat {
    value: u8,
}

impl Stat {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn from_bus(bus: &Bus) -> Self {
        Self {
            value: bus.read(IOType::STAT.address()),
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

    pub fn mode(&self) -> u8 {
        self.value & 0x03
    }

    fn interrupt_line(&self) -> bool {
        (self.lyc_eq_ly_select() && self.lyc_eq_ly())
            || (self.mode_0_select() && self.mode() == 0)
            || (self.mode_1_select() && self.mode() == 1)
            || (self.mode_2_select() && self.mode() == 2)
    }

    pub(crate) fn stat_transformer((old_value, new_value): (u8, u8)) -> Option<u8> {
        Some((old_value & 0x07) | (new_value & !0x07))
    }

    pub(crate) fn on_ly_write(bus: &mut Bus, value: u8) {
        Stat::update_stat(bus, value, bus.read(IOType::LYC.address()));
    }

    pub(crate) fn on_lyc_write(bus: &mut Bus, value: u8) {
        Stat::update_stat(bus, bus.read(IOType::LY.address()), value);
    }

    fn update_stat(bus: &mut Bus, ly: u8, lyc: u8) {
        let bit_2 = if lyc == ly { 0x04 } else { 0x00 };
        let stat = bus.read(IOType::STAT.address());
        let new_stat = (stat & !0x04) | bit_2;
        if new_stat != stat {
            bus.io_registers
                .write_unchecked(IOType::STAT.address(), new_stat);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StatInterruptDetector {
    should_trigger_interrupt: bool,
    edge_detector: EdgeDetector,
}

impl StatInterruptDetector {
    pub fn new() -> Self {
        Self {
            edge_detector: EdgeDetector::new(EdgeMode::Rising),
            should_trigger_interrupt: false,
        }
    }

    pub fn tick(&mut self, new_stat: u8) {
        let interrupt_line = Stat::new(new_stat).interrupt_line();
        self.should_trigger_interrupt = self.edge_detector.tick(interrupt_line);
    }

    pub fn should_trigger_interrupt(&mut self) -> bool {
        self.should_trigger_interrupt
    }

    pub(crate) fn on_stat_write(&mut self, bus: &mut Bus, value: u8) {
        self.tick(value);
        if self.should_trigger_interrupt() {
            bus.request_interrupt(InterruptType::Lcd);
        }
    }
}

impl Default for StatInterruptDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl yagber_app::Component for StatInterruptDetector {}
