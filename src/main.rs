use yagber_app::Emulator;
use yagber_display::WinitRunner;

fn main() {
    yagber::init_tracing();

    // Order matters, some plugins depend on others
    // TODO: Remove this foot gun
    Emulator::new()
        // Memory must be first
        .with_plugin(yagber_memory::MemoryPlugin::default())
        .with_plugin(yagber_cpu::CpuPlugin)
        .with_plugin(yagber_ppu::PpuPlugin)
        .with_plugin(yagber_link_cable::LinkCablePlugin::default().with_serial_output_stdout())
        .with_plugin(yagber_display::DisplayPlugin)
        // Timer must be last
        .with_plugin(yagber_timer::TimerPlugin)
        .run::<WinitRunner>();
}
