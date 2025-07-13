mod dma;
mod hdma;

pub struct DmaPlugin;

impl yagber_app::Plugin for DmaPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(dma::Dma::new())
            .with_component(hdma::Hdma::new())
            .on_mcycle(dma::Dma::on_mcycle);

        let dma_hook = emulator.attach_component(dma::Dma::on_dma_write);
        let hdma_len_hook = emulator.attach_components2(hdma::Hdma::on_hdma_len_write);
        let hdma_reader = emulator.attach_component(hdma::Hdma::hdma_len_reader);
        let hdma_stat_hook = emulator.attach_components2(hdma::Hdma::on_stat_write);

        emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("Bus component missing")
            .io_registers
            .with_hook(yagber_memory::IOType::DMA, dma_hook)
            .with_hook(yagber_memory::IOType::HdmaLen, hdma_len_hook)
            .with_reader(yagber_memory::IOType::HdmaLen, hdma_reader)
            .with_hook(yagber_memory::IOType::STAT, hdma_stat_hook);
    }
}
