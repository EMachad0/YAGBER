pub struct Dma {
    enabled: bool,
    cycles: u32,
    source_addr: u16,
}

impl Dma {
    const DMA_ADDR: u16 = 0xFF46;
    const DMA_DELAY_CYCLES: u32 = 160;
    const DMA_TARGET_ADDR: u16 = 0xFE00;

    pub fn new() -> Self {
        Self {
            enabled: false,
            cycles: 0,
            source_addr: 0,
        }
    }

    pub fn on_memory_write(
        emulator: &mut yagber_app::Emulator,
        event: &yagber_memory::MemoryWriteEvent,
    ) {
        let dma = emulator
            .get_component_mut::<Dma>()
            .expect("DMA component missing");
        if event.address == Self::DMA_ADDR {
            dma.start(event.value as u16 * 0x100);
        }
    }

    fn start(&mut self, source_addr: u16) {
        self.enabled = true;
        self.cycles = 0;
        self.source_addr = source_addr;
    }

    pub fn on_mcycle(emulator: &mut yagber_app::Emulator, _event: &yagber_app::MCycleEvent) {
        let (dma, bus) = emulator
            .get_components_mut2::<Dma, yagber_memory::Bus>()
            .expect("DMA and/or Bus component missing");
        if dma.enabled {
            dma.step(bus);
        }
    }

    fn step(&mut self, bus: &mut yagber_memory::Bus) {
        self.cycles += 1;
        if self.cycles == Self::DMA_DELAY_CYCLES {
            self.perform_transfer(bus);
            self.disable();
        }
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn perform_transfer(&mut self, bus: &mut yagber_memory::Bus) {
        let source_addr = self.source_addr;
        let target_addr = Self::DMA_TARGET_ADDR;

        for i in 0..0xA0 {
            let value = bus.read(source_addr + i);
            bus.write(target_addr + i, value);
        }
    }
}
