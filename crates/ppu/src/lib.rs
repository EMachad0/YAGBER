mod dma;
mod ppu;
mod ppu_mode;
mod ppu_mode_observer;
mod tile;

#[macro_use]
extern crate tracing;

pub struct PpuPlugin;

impl yagber_app::Plugin for PpuPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(dma::Dma::new())
            .with_event_handler::<yagber_memory::MemoryWriteEvent>(dma::Dma::on_memory_write)
            .with_event_handler::<yagber_app::MCycleEvent>(dma::Dma::on_mcycle)
            .with_component(ppu::Ppu::new())
            .with_event_handler::<yagber_app::DotCycleEvent>(ppu::Ppu::on_dot_cycle)
            .with_event_handler::<yagber_memory::MemoryWriteEvent>(
                ppu_mode_observer::PpuModeObserver::on_memory_write,
            );
    }
}
