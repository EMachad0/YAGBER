use yagber_cpu::Cpu;
use yagber_ppu::Ppu;
use yagber_ram::Ram;

#[derive(Debug, Default)]
pub struct Emulator {
    cycles: u64,
    cpu: Cpu,
    ppu: Ppu,
    ram: Ram,
}

impl Emulator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load the ram with the cartridge
    pub fn with_cartridge(mut self, rom: &[u8]) -> Self {
        // copy rom header
        self.ram.load_rom(rom);
        self
    }

    fn step(&mut self) {
        let ram = &mut self.ram;
        if self.cycles % 4 == 0 {
            self.cpu.step(ram);
            check_serial_output(ram);
        }
        self.ppu.step(ram);
        self.cycles += 1;
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }

    pub fn run_for(&mut self, cycles: u64) {
        while self.cycles < cycles {
            self.step();
        }
    }
}

pub fn check_serial_output(ram: &mut Ram) {
    let serial_output = ram.read(0xFF01);
    let serial_control = ram.read(0xFF02);
    if serial_control & 0x81 != 0 {
        tracing::info!("Serial output: {}", serial_output as char);
        ram.write(0xFF02, serial_control & !0x81);
        ram.request_interrupt(yagber_ram::InterruptType::Serial);
    }
}
