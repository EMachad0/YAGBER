mod ram_observer;
mod timer;

#[macro_use]
extern crate tracing;

pub use ram_observer::DivObserver;
pub use timer::Timer;

pub struct TimerPlugin;

impl yagber_app::Plugin for TimerPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(Timer::new())
            .with_event_handler::<yagber_app::MCycleEvent>(Timer::on_mcycle)
            .with_event_handler::<yagber_memory::MemoryWriteEvent>(DivObserver::on_memory_write);
    }
}
