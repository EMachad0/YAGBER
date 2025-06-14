pub use yagber_app as app;
pub use yagber_clock as clock;
pub use yagber_cpu as cpu;
pub use yagber_memory as ram;
pub use yagber_ppu as ppu;

pub use yagber_app::Emulator;

pub fn init_tracing() {
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .event_format(tracing_subscriber::fmt::format().compact())
        .try_init();
}
