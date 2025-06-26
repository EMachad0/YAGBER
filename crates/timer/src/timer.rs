use yagber_memory::{Bus, InterruptType};

use crate::edge_detector::{EdgeDetector, EdgeMode};

pub const DIV_ADDR: u16 = 0xFF04;
pub const TIMA_ADDR: u16 = 0xFF05;
pub const TMA_ADDR: u16 = 0xFF06;
pub const TAC_ADDR: u16 = 0xFF07;

/// System Counter is a 16.777216 MHz clock.
/// Incremented every M-Cycle.
#[derive(Debug, Clone, Copy)]
pub struct SystemCounter {
    /// 14 bits.
    m_cycles: u16,
    tac_edge_detector: EdgeDetector,
}

impl SystemCounter {
    pub fn new() -> Self {
        Self {
            m_cycles: 0,
            tac_edge_detector: EdgeDetector::new(EdgeMode::Falling),
        }
    }

    pub fn tick(&mut self) {
        self.m_cycles = self.m_cycles.wrapping_add(1);
    }

    /// Div is the visible part of the system counter.
    pub fn div(&self) -> u8 {
        (self.m_cycles >> 6) as u8
    }

    /// Returns if TIMA should be incremented based on the TAC frequency.
    pub fn tac_cycle(&mut self, bit: u8) -> bool {
        let bit_value = (self.m_cycles & (1u16 << bit)) != 0;
        self.tac_edge_detector.tick(bit_value)
    }

    #[cfg(test)]
    pub(crate) fn cycles(&self) -> u16 {
        self.m_cycles
    }

    #[cfg(test)]
    pub(crate) fn from_cycles(cycles: u16) -> Self {
        let edge_detector = EdgeDetector::new(EdgeMode::Falling);
        Self {
            m_cycles: cycles,
            tac_edge_detector: edge_detector,
        }
    }
}

impl Default for SystemCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Timer {
    system_counter: SystemCounter,
    tima_overflow: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            system_counter: SystemCounter::new(),
            tima_overflow: false,
        }
    }

    pub fn on_mcycle(emulator: &mut yagber_app::Emulator, _event: &yagber_app::MCycleEvent) {
        let (timer, bus) = emulator
            .get_components_mut2::<Timer, Bus>()
            .expect("Timer and/or Bus component missing");

        timer.tick(bus);
    }

    /// Tick the timer.
    /// Represents a single M-Cycle.
    /// Meant to be called after executing the instruction.
    pub fn tick(&mut self, ram: &mut Bus) {
        self.system_counter.tick();
        let div = self.system_counter.div();
        self.write_div(ram, div);

        let tima = self.read_tima(ram);
        if self.tima_overflow {
            let tma = self.read_tma(ram);
            self.write_tima(ram, tma);
            trace!("Timer overflow requesting Timer interrupt");
            ram.request_interrupt(InterruptType::Timer);
            self.tima_overflow = false;
        }

        let tac = self.read_tac(ram);
        let bit = Self::get_tac_sys_bit(tac);
        if self.tac_enabled(tac) && self.system_counter.tac_cycle(bit) {
            let tima = tima.checked_add(1).unwrap_or_else(|| {
                self.tima_overflow = true;
                0
            });

            self.write_tima(ram, tima);
        }
    }

    fn tac_enabled(&self, tac: u8) -> bool {
        (tac & (1 << 2)) != 0
    }

    /// Returns the bit that is used to determine the frequency of the timer.
    /// Cycle frequency is determined by the bit times two due to using a falling edge detector.
    fn get_tac_sys_bit(tac: u8) -> u8 {
        match tac & 0b11 {
            0b00 => 7, // Every 256 M-Cycles
            0b01 => 1, // Every 4 M-Cycles
            0b10 => 3, // Every 16 M-Cycles
            0b11 => 5, // Every 64 M-Cycles
            _ => unreachable!("Invalid TAC mode: {}", tac),
        }
    }

    fn read_tima(&self, ram: &Bus) -> u8 {
        ram.read(TIMA_ADDR)
    }

    fn read_tma(&self, ram: &Bus) -> u8 {
        ram.read(TMA_ADDR)
    }

    fn read_tac(&self, ram: &Bus) -> u8 {
        ram.read(TAC_ADDR)
    }

    fn write_tima(&mut self, ram: &mut Bus, value: u8) {
        ram.write(TIMA_ADDR, value);
    }

    fn write_div(&mut self, ram: &mut Bus, value: u8) {
        ram.write(DIV_ADDR, value);
    }

    #[cfg(test)]
    pub(crate) fn from_cycles(cycles: u16, tac: u8) -> Self {
        let bit = Self::get_tac_sys_bit(tac);
        let mut system_counter = SystemCounter::from_cycles(cycles);
        // Tick the system counter to set the edge detector to the correct value
        system_counter.tac_cycle(bit);
        Self {
            system_counter,
            tima_overflow: false,
        }
    }

    #[cfg(test)]
    pub(crate) fn cycles(&self) -> u16 {
        self.system_counter.cycles()
    }
}

impl yagber_app::Component for Timer {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_overflow_behavior() {
        let tac = 0xFD; // Timer ON, Every 4 M-Cycles
        let mut timer = Timer::from_cycles(0x2B, tac);
        let mut bus = Bus::default();
        let if_addr = 0xFF0F;

        bus.write(TIMA_ADDR, 0xFE);
        bus.write(TMA_ADDR, 0x23);
        bus.write(TAC_ADDR, tac);
        bus.write(if_addr, 0xE0); // IF

        assert_eq!(timer.cycles(), 0x2B);
        assert_eq!(timer.read_tima(&bus), 0xFE);
        assert_eq!(timer.read_tma(&bus), 0x23);
        assert_eq!(bus.read(if_addr), 0xE0);

        // first falling edge
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x2C);
        assert_eq!(timer.read_tima(&bus), 0xFF);
        assert_eq!(bus.read(if_addr), 0xE0);

        // rising edge
        timer.tick(&mut bus);
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x2E);
        assert_eq!(timer.read_tima(&bus), 0xFF);
        assert_eq!(bus.read(if_addr), 0xE0);

        // second falling edge
        timer.tick(&mut bus);
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x30);
        assert_eq!(timer.read_tima(&bus), 0x00);
        assert_eq!(bus.read(if_addr), 0xE0);

        // no edge
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x31);
        assert_eq!(timer.read_tima(&bus), 0x23);
        assert_eq!(bus.read(if_addr), 0xE4);

        // second rising edge
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x32);
        assert_eq!(timer.read_tima(&bus), 0x23);
        assert_eq!(bus.read(if_addr), 0xE4);

        // third falling edge
        timer.tick(&mut bus);
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x34);
        assert_eq!(timer.read_tima(&bus), 0x24);
        assert_eq!(bus.read(if_addr), 0xE4);
        assert_eq!(timer.read_tma(&bus), 0x23)
    }
}
