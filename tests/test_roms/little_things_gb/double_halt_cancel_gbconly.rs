use std::fs;
use yagber::Emulator;

#[test]
fn test_little_things_gb_double_halt_cancel() {
    yagber::init_tracing();

    let rom_path = "test_roms/little-things-gb/double-halt-cancel-gbconly.gb";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");

    let out_log_path = format!("out/{}.log", rom_path);

    let rom = fs::read(rom_path).expect("Failed to read ROM");
    let mut emu = Emulator::new()
        .with_cartridge(&rom)
        .with_serial_output_file(&out_log_path)
        .with_serial_output_buffer();

    // Run emulation for some steps
    emu.run_for(1_000_000_000);
}
