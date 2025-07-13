mod models;
mod ppu;
mod ppu_mode;

pub struct PpuPlugin;

impl yagber_app::Plugin for PpuPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(ppu::Ppu::new())
            .on_dot_cycle(ppu::Ppu::on_dot_cycle);
    }
}
