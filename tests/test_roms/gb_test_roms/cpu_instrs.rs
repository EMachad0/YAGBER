use std::fs;
use yet_another_gb_rust_emulator::cpu::Cpu;

#[test]
fn test_blargg_cpu_instrs() {
    yet_another_gb_rust_emulator::init_tracing();

    let rom_path = "test_roms/gb-test-roms/cpu_instrs/cpu_instrs.gb";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");

    let rom = fs::read(rom_path).expect("Failed to read ROM");
    let mut emu = Cpu::from_rom(rom);

    // Run emulation for some steps
    for _ in 0..10 {
        emu.step();
    }

    // Inspect memory or output for test success
    // let result = emu.memory.read_byte(0xFF02);
    // assert_eq!(result, 0x01, "Blargg test failed!");
}
