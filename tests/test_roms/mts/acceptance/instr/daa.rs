use std::fs;
use yagber::Emulator;

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

    // Run emulation for some steps
    emu.run_for(10_000_000);

    let output_buffer = emu.get_serial_output_buffer().unwrap();
    // Check the output buffer
    let expected_output = &[3, 5, 8, 13, 21, 34];

    assert_eq!(
        output_buffer, expected_output,
        "Output buffer does not match expected output"
    );
}
