mod ppu;
mod ppu_mode;
mod ppu_mode_observer;
mod scan_line;
mod tile;

#[macro_use]
extern crate tracing;

pub use ppu::Ppu;
pub use ppu_mode_observer::PpuModeObserver;

pub struct PpuPlugin;

impl yagber_app::Plugin for PpuPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(Ppu::new())
            .with_event_handler::<yagber_app::DotCycleEvent>(Ppu::on_dot_cycle)
            .with_event_handler::<yagber_memory::MemoryWriteEvent>(
                PpuModeObserver::on_memory_write,
            );
    }
}
