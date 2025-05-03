use std::fs;
use yagber::Emulator;

use crate::mts::run_emulator;

#[test]
fn test_mts_daa() {
    yagber::init_tracing();

    let rom_path = "test_roms/mts/acceptance/instr/daa.gb";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");

    let out_log_path = format!("out/{}.log", rom_path);

    let rom = fs::read(rom_path).expect("Failed to read ROM");
    let mut emu = Emulator::new()
        .with_cartridge(&rom)
        .with_serial_output_file(&out_log_path)
        .with_serial_output_buffer();

    let status = run_emulator(&mut emu);

    let output_buffer = emu.get_serial_output_buffer().unwrap();

    assert!(status.is_success(), "Output buffer:\n{:?}", output_buffer);
}
