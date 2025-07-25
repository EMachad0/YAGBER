use yagber_app::{EdgeDetector, EdgeMode};
use yagber_memory::{Bus, IOType, InterruptType};

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
        self.m_cycles += 1;
        if self.m_cycles >= (1 << 14) {
            self.m_cycles = 0;
        }
    }

    /// Div is the visible part of the system counter.
    pub fn div(&self) -> u8 {
        (self.m_cycles >> 6) as u8
    }

    /// Returns if TIMA should be incremented based on the TAC frequency.
    pub fn tima_should_increment(&mut self, tac_clock: yagber_memory::TacClock) -> bool {
        let bit_value = (self.m_cycles & tac_clock.div_mask()) != 0;
        self.tac_edge_detector.tick(bit_value)
    }

    pub fn reset(&mut self) {
        self.m_cycles = 0;
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

    pub fn on_mcycle(emulator: &mut yagber_app::Emulator) {
        let (timer, bus) = emulator
            .get_components_mut2::<Timer, Bus>()
            .expect("Timer and/or Bus component missing");

        timer.tick(bus);
    }

    /// Tick the timer.
    /// Represents a single M-Cycle.
    /// Meant to be called after executing the instruction.
    pub fn tick(&mut self, bus: &mut Bus) {
        self.system_counter.tick();
        let div = self.system_counter.div();
        self.write_div(bus, div);

        let tima = self.read_tima(bus);
        if self.tima_overflow {
            let tma = self.read_tma(bus);
            self.write_tima(bus, tma);
            bus.request_interrupt(InterruptType::Timer);
            self.tima_overflow = false;
        }

        let tac = yagber_memory::TacRegister::from_bus(bus);
        let tac_clock = tac.clock_select();
        if tac.enabled() && self.system_counter.tima_should_increment(tac_clock) {
            let tima = tima.checked_add(1).unwrap_or_else(|| {
                self.tima_overflow = true;
                0
            });

            self.write_tima(bus, tima);
        }
    }

    fn read_tima(&self, bus: &Bus) -> u8 {
        bus.io_registers.read(IOType::TIMA.address())
    }

    fn read_tma(&self, bus: &Bus) -> u8 {
        bus.io_registers.read(IOType::TMA.address())
    }

    fn write_tima(&mut self, bus: &mut Bus, value: u8) {
        bus.io_registers.write(IOType::TIMA.address(), value);
    }

    fn write_div(&mut self, bus: &mut Bus, value: u8) {
        let old_div = bus.io_registers.read(IOType::DIV.address());
        if old_div != value {
            bus.io_registers
                .write_unhooked(IOType::DIV.address(), value);
        }
    }

    pub fn reset(&mut self) {
        self.system_counter.reset();
        self.tima_overflow = false;
    }

    pub(crate) fn on_div_write(&mut self, _value: u8) {
        self.reset();
    }

    #[cfg(test)]
    pub(crate) fn from_cycles(cycles: u16, tac: u8) -> Self {
        let tac_clock = yagber_memory::TacRegister::new(tac).clock_select();
        let mut system_counter = SystemCounter::from_cycles(cycles);
        // Tick the system counter to set the edge detector to the correct value
        system_counter.tima_should_increment(tac_clock);
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

        bus.io_registers.write(IOType::TIMA.address(), 0xFE);
        bus.io_registers.write(IOType::TMA.address(), 0x23);
        bus.io_registers.write(IOType::TAC.address(), tac);
        bus.io_registers.write(IOType::IF.address(), 0xE0); // IF

        assert_eq!(timer.cycles(), 0x2B);
        assert_eq!(timer.read_tima(&bus), 0xFE);
        assert_eq!(timer.read_tma(&bus), 0x23);
        assert_eq!(bus.io_registers.read(IOType::IF.address()), 0xE0);

        // first falling edge
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x2C);
        assert_eq!(timer.read_tima(&bus), 0xFF);
        assert_eq!(bus.io_registers.read(IOType::IF.address()), 0xE0);

        // rising edge
        timer.tick(&mut bus);
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x2E);
        assert_eq!(timer.read_tima(&bus), 0xFF);
        assert_eq!(bus.io_registers.read(IOType::IF.address()), 0xE0);

        // second falling edge
        timer.tick(&mut bus);
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x30);
        assert_eq!(timer.read_tima(&bus), 0x00);
        assert_eq!(bus.io_registers.read(IOType::IF.address()), 0xE0);

        // no edge
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x31);
        assert_eq!(timer.read_tima(&bus), 0x23);
        assert_eq!(bus.io_registers.read(IOType::IF.address()), 0xE4);

        // second rising edge
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x32);
        assert_eq!(timer.read_tima(&bus), 0x23);
        assert_eq!(bus.io_registers.read(IOType::IF.address()), 0xE4);

        // third falling edge
        timer.tick(&mut bus);
        timer.tick(&mut bus);

        assert_eq!(timer.cycles(), 0x34);
        assert_eq!(timer.read_tima(&bus), 0x24);
        assert_eq!(bus.io_registers.read(IOType::IF.address()), 0xE4);
        assert_eq!(timer.read_tma(&bus), 0x23)
    }
}
