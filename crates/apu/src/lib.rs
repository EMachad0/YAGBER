mod apu;
mod channels;
mod divapu;

pub use apu::Apu;

pub struct ApuPlugin;

impl yagber_app::Plugin for ApuPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        let div_apu = divapu::DivApu::new();
        let apu = apu::Apu::new();

        emulator
            .with_component(div_apu)
            .on_mcycle(divapu::DivApu::on_mcycle)
            .with_component(apu)
            .on_tcycle(apu::Apu::on_tcycle);

        let apu_aud1high_hook = emulator.attach_components2(apu::Apu::on_aud_1_high_write);

        emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("Bus component not found")
            .io_registers
            .with_hook(yagber_memory::IOType::AUD1HIGH, apu_aud1high_hook);
    }
}
