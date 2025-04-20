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

    /// Load the ram with the boot ROM
    pub fn with_boot_rom(mut self) -> Self {
        // Initialize the CPU with the boot ROM
        let path = "resources/cgb_boot.bin";
        let boot_rom =
            std::fs::read(path).unwrap_or_else(|_| panic!("Failed to read boot ROM from {}", path));

        // CGB boot ROM is split into two parts
        // 0x0000–0x00FF: CGB boot ROM
        // 0x0100–0x08FF: CGB boot ROM (bank 0)
        // The cartridge Header is at 0x0100–0x014F (which is in the middle of the boot ROM)
        // On this boot room, the cartridge header starts as zeroes
        self.ram
            .copy_from_slice(0x0000..0x08FF, &boot_rom[0x0000..0x08FF]);

        self
    }

    /// Load the ram with the cartridge
    pub fn with_cartridge(mut self, rom: &[u8]) -> Self {
        // copy rom header
        self.ram
            .copy_from_slice(0x0100..0x014F, &rom[0x0100..0x014F]);
        self
    }

    fn step(&mut self) {
        let ram = &mut self.ram;
        if self.cycles % 4 == 0 {
            self.cpu.step(ram);
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
