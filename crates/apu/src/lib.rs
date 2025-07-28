mod apu;
mod audio_buffer;
mod channels;
mod divapu;
mod high_pass_filter;
mod sweep;

pub use apu::Apu;
pub use audio_buffer::{AudioBuffer, ConsumerCache, ProducerCache};

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

        use channels::PulseChannel;
        let sweep_aud1sweep_hook = emulator.attach_component(sweep::Sweep::on_aud_1_sweep_write);
        let sweep_aud1high_hook = emulator.attach_components2(sweep::Sweep::on_aud_1_high_write);
        let ch1_aud1high_hook = emulator.attach_components2(PulseChannel::on_aud_1_high_write);
        let ch1_aud1env_hook = emulator.attach_component(PulseChannel::on_aud_1_env_write);
        let ch2_aud2high_hook = emulator.attach_components2(PulseChannel::on_aud_2_high_write);
        let ch2_aud2env_hook = emulator.attach_component(PulseChannel::on_aud_2_env_write);

        emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("Bus component not found")
            .io_registers
            .with_hook(yagber_memory::IOType::AUD1SWEEP, sweep_aud1sweep_hook)
            .with_hook(yagber_memory::IOType::AUD1HIGH, sweep_aud1high_hook)
            .with_hook(yagber_memory::IOType::AUD1HIGH, ch1_aud1high_hook)
            .with_hook(yagber_memory::IOType::AUD1ENV, ch1_aud1env_hook)
            .with_hook(yagber_memory::IOType::AUD2HIGH, ch2_aud2high_hook)
            .with_hook(yagber_memory::IOType::AUD2ENV, ch2_aud2env_hook);
    }
}
