fn main() {
    let rom_path = std::env::args().nth(1).expect("No ROM path provided");
    let rom = std::fs::read(rom_path).expect("Failed to read ROM file");

    let mut emulator = yagber_app::Emulator::new();

    if cfg!(feature = "trace") {
        // Log must be first
        emulator = emulator.with_plugin(yagber_log::LogPlugin::default());
    }

    // Order matters
    emulator
        // Memory must be first
        .with_plugin(yagber_memory::MemoryPlugin::default().with_cartridge(&rom))
        .with_plugin(yagber_cpu::CpuPlugin)
        .with_plugin(yagber_ppu::PpuPlugin)
        .with_plugin(yagber_dma::DmaPlugin)
        .with_plugin(yagber_link_cable::LinkCablePlugin::default().with_serial_output_stdout())
        .with_plugin(yagber_display::DisplayPlugin)
        .with_plugin(yagber_input::InputPlugin)
        .with_plugin(yagber_cpal::CpalPlugin)
        // Timer must be last
        .with_plugin(yagber_timer::TimerPlugin)
        .run::<yagber_display::WinitRunner>();
}
