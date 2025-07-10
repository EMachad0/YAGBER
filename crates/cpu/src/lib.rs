mod alu;
mod cpu;
mod ime;
mod instructions;
mod registers;

pub use cpu::Cpu;

pub struct CpuPlugin;

impl yagber_app::Plugin for CpuPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(Cpu::default())
            .on_mcycle(Cpu::on_mcycle);
    }
}
