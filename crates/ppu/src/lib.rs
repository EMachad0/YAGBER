mod dma;
mod models;
mod ppu;
mod ppu_mode;

pub struct PpuPlugin;

impl yagber_app::Plugin for PpuPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        let emulator_ptr = emulator as *mut yagber_app::Emulator;

        let bus = emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("Bus component missing");
        bus.io_registers
            .add_hook(yagber_memory::IOType::DMA, move |value| {
                dma::Dma::on_dma_write(unsafe { &mut *emulator_ptr }, value)
            });

        emulator
            .with_component(dma::Dma::new())
            .on_mcycle(dma::Dma::on_mcycle)
            .with_component(ppu::Ppu::new())
            .on_dot_cycle(ppu::Ppu::on_dot_cycle);
    }
}
