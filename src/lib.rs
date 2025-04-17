pub use yet_another_gb_rust_emulator_cpu as cpu;

pub fn init_tracing() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .event_format(tracing_subscriber::fmt::format().pretty())
        .init();
}
