mod dma;
mod models;
mod ppu;
mod ppu_mode;
mod ppu_mode_observer;

pub struct PpuPlugin;

impl yagber_app::Plugin for PpuPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(dma::Dma::new())
            .with_event_handler(dma::Dma::on_memory_write)
            .on_mcycle(dma::Dma::on_mcycle)
            .with_component(ppu::Ppu::new())
            .on_dot_cycle(ppu::Ppu::on_dot_cycle)
            .with_event_handler(ppu_mode_observer::PpuModeObserver::on_memory_write);
    }
}
