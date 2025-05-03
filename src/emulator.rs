use yagber_cpu::Cpu;
use yagber_link_cable::LinkCable;
use yagber_ppu::Ppu;
use yagber_ram::Ram;

#[derive(Debug, Default)]
pub struct Emulator {
    cycles: u64,
    cpu: Cpu,
    ppu: Ppu,
    ram: Ram,
    link_cable: LinkCable,
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
        }
        self.ppu.step(ram);
        self.link_cable.step(ram);

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
}
