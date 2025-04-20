use std::fs;
use yagber::Emulator;

#[test]
fn test_blargg_cpu_instrs() {
    yagber::init_tracing();

    let rom_path = "test_roms/gb-test-roms/cpu_instrs/individual/01-special.gb";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");

    let rom = fs::read(rom_path).expect("Failed to read ROM");
    let mut emu = Emulator::new().with_boot_rom().with_cartridge(&rom);

    // Run emulation for some steps
    emu.run_for(1_000_000_000);
}
