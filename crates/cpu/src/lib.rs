mod alu;
mod cpu;
mod ime;
mod instructions;
mod registers;

#[macro_use]
extern crate tracing;

pub use cpu::Cpu;

pub struct CpuPlugin;

impl yagber_app::Plugin for CpuPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(Cpu::default())
            .with_event_handler(Cpu::on_mcycle);
    }
}
