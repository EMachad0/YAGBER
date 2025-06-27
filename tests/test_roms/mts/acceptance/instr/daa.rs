use std::fs;

use crate::mts::run_emulator;

#[test]
fn test_mts_daa() {
    let rom_path = "test_roms/mts/acceptance/instr/daa.gb";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");

    let out_log_path = format!("out/{}.log", rom_path);

    let rom = fs::read(rom_path).expect("Failed to read ROM");

    let status = run_emulator(&rom, &out_log_path);

    assert!(
        status.result.is_success(),
        "Output buffer:\n{}",
        status.output_buffer
    );
}
