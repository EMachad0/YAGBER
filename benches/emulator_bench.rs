const TARGET_FRAMES: u32 = 128;

fn build_headless_emulator() -> yagber::Emulator {
    // Headless setup with all core functionality
    yagber::Emulator::new()
        .with_plugin(yagber_memory::MemoryPlugin::default())
        .with_plugin(yagber_cpu::CpuPlugin)
        .with_plugin(yagber_ppu::PpuPlugin)
        .with_plugin(yagber_apu::ApuPlugin)
        .with_plugin(yagber_dma::DmaPlugin)
        .with_plugin(yagber_link_cable::LinkCablePlugin::default().with_serial_output_stdout())
        .with_plugin(yagber_input::InputPlugin)
        .with_plugin(yagber_timer::TimerPlugin)
}

fn run_frames(emulator: &mut yagber::Emulator, frames: u32) {
    let dots_per_frame = yagber_ppu::Ppu::DOTS_PER_FRAME;
    let total_dots = dots_per_frame.saturating_mul(frames);
    for _ in 0..total_dots {
        emulator.step();
    }
}

fn bench_emulate_frames(c: &mut criterion::Criterion) {
    c.bench_function("emulate_frames", |b| {
        b.iter_batched(
            build_headless_emulator,
            |mut emulator| {
                run_frames(&mut emulator, TARGET_FRAMES);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion::criterion_group!(benches, bench_emulate_frames);
criterion::criterion_main!(benches);
