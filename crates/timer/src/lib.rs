mod timer;

pub use timer::Timer;

pub struct TimerPlugin;

impl yagber_app::Plugin for TimerPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(Timer::new())
            .on_mcycle(Timer::on_mcycle);

        let timer_div_hook = emulator.attach_component(timer::Timer::on_div_write);

        emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("Bus component missing")
            .io_registers
            .add_hook(yagber_memory::IOType::DIV, timer_div_hook);
    }
}
