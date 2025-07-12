mod dma;
mod models;
mod ppu;
mod ppu_mode;

pub struct PpuPlugin;

impl yagber_app::Plugin for PpuPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(dma::Dma::new())
            .on_mcycle(dma::Dma::on_mcycle)
            .with_component(ppu::Ppu::new())
            .on_dot_cycle(ppu::Ppu::on_dot_cycle);

        let dma_hook = emulator.attach_component(dma::Dma::on_dma_write);

        emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("Bus component missing")
            .io_registers
            .add_hook(yagber_memory::IOType::DMA, dma_hook);
    }
}
