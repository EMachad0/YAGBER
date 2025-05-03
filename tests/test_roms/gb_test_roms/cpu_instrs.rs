use std::fs;
use yagber::Emulator;

#[test]
fn test_blargg_cpu_instrs() {
    yagber::init_tracing();

    let rom_path = "test_roms/gb-test-roms/cpu_instrs/cpu_instrs.gb";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");

    let out_log_path = format!("out/{}.log", rom_path);

    let rom = fs::read(rom_path).expect("Failed to read ROM");
    let mut emu = Emulator::new()
        .with_cartridge(&rom)
        .with_serial_output_file(&out_log_path)
        .with_serial_output_buffer();

    // Run emulation for some steps
    emu.run_for(10_000_000);
}
