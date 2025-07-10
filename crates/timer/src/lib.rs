mod ram_observer;
mod timer;

pub use ram_observer::DivObserver;
pub use timer::Timer;

pub struct TimerPlugin;

impl yagber_app::Plugin for TimerPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(Timer::new())
            .on_mcycle(Timer::on_mcycle)
            .with_event_handler(DivObserver::on_memory_write);
    }
}
