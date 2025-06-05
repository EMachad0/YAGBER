use yagber_cpu::Cpu;
use yagber_link_cable::LinkCable;
use yagber_memory::Bus;
use yagber_ppu::Ppu;
use yagber_timer::Timer;

use crate::runners::Runner;

#[derive(Debug)]
pub struct Emulator {
    cycles: u64,
    cpu: Cpu,
    ppu: Ppu,
    bus: Bus,
    link_cable: LinkCable,
    timer: Timer,
}

impl Emulator {
    pub fn new() -> Self {
        let mut bus = Bus::default();

        bus.add_observer(yagber_timer::DivObserver::new());
        bus.add_observer(yagber_ppu::PpuModeObserver::new());

        Self {
            cycles: 0,
            bus,
            cpu: Cpu::default(),
            ppu: Ppu::default(),
            link_cable: LinkCable::default(),
            timer: Timer::default(),
        }
    }

    /// Load the ram with the cartridge
    pub fn with_cartridge(mut self, rom: &[u8]) -> Self {
        // copy rom header
        self.bus.load_rom(rom);
        self
    }

    pub fn step(&mut self) {
        let is_m_cycle = self.is_m_cycle();
        let ram = &mut self.bus;

        if is_m_cycle {
            self.cpu.step(ram);
        }
        // PPU runs every T-Cycle or dot
        self.ppu.step(ram);

        self.link_cable.step(ram);

        if is_m_cycle {
            // Timer must be ticked after executing the instruction
            self.timer.tick(ram);
        }

        self.cycles += 1;
    }

    pub fn run<T: Runner>(self) {
        let mut runner = T::new(self);
        runner.run();
    }

    pub fn run_for(&mut self, cycles: u64) {
        while self.cycles < cycles {
            self.step();
        }
    }

    pub fn with_serial_output_file(mut self, path: &str) -> Self {
        self.link_cable.to_file(path);
        self
    }

    pub fn with_serial_output_buffer(mut self) -> Self {
        self.link_cable.to_buffer();
        self
    }

    pub fn with_serial_output_stdout(mut self) -> Self {
        self.link_cable.to_stdout();
        self
    }

    pub fn get_serial_output_buffer(&self) -> Option<&[u8]> {
        self.link_cable.get_buffer()
    }

    pub fn get_cycles(&self) -> u64 {
        self.cycles
    }

    fn is_m_cycle(&self) -> bool {
        self.cycles % 4 == 0
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}
