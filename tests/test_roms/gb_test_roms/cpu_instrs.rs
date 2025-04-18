use std::fs;
use yet_another_gb_rust_emulator::cpu::Cpu;

#[test]
fn test_blargg_cpu_instrs() {
    yet_another_gb_rust_emulator::init_tracing();

    let rom_path = "test_roms/gb-test-roms/cpu_instrs/individual/01-special.gb";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");

    let rom = fs::read(rom_path).expect("Failed to read ROM");
    let mut emu = Cpu::new().with_rom(&rom);

    // Run emulation for some steps
    for _ in 0..100_000 {
        emu.step();
    }
}
