mod dma;

pub struct DmaPlugin;

impl yagber_app::Plugin for DmaPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(dma::Dma::new())
            .on_mcycle(dma::Dma::on_mcycle);

        let dma_hook = emulator.attach_component(dma::Dma::on_dma_write);

        emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("Bus component missing")
            .io_registers
            .add_hook(yagber_memory::IOType::DMA, dma_hook);
    }
}
