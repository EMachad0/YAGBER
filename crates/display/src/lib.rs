mod display;
mod winit_app;
mod winit_runner;

pub use display::Display;
pub use winit_runner::WinitRunner;

pub struct DisplayPlugin;

impl yagber_app::Plugin for DisplayPlugin {
    fn init(self, _emulator: &mut yagber_app::Emulator) {
        // TODO: Implement
    }
}
