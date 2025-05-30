use std::{fs, path::PathBuf};
use yagber::Emulator;

use crate::gb_test_roms::run_emulator;

const ROM_PATH: &str = "test_roms/gb-test-roms/cpu_instrs/cpu_instrs.gb";
const INDV_ROM_PATH: &str = "test_roms/gb-test-roms/cpu_instrs/individual/";

#[test]
fn test_blargg_cpu_instrs() {
    yagber::init_tracing();

    assert!(fs::metadata(ROM_PATH).is_ok(), "Test ROM not found!");

    let out_log_path = format!("out/{}.log", ROM_PATH);

    let rom = fs::read(ROM_PATH).expect("Failed to read ROM");
    let mut emu = Emulator::new()
        .with_cartridge(&rom)
        .with_serial_output_file(&out_log_path)
        .with_serial_output_buffer();

    let status = run_emulator(&mut emu);
    let output_buffer = emu.get_serial_output_buffer().unwrap();
    assert!(
        status.is_success(),
        "Output buffer:\n{}",
        String::from_utf8_lossy(output_buffer)
    );
}

#[test]
fn test_blargg_cpu_instrs_01_special() {
    yagber::init_tracing();

    let test_name = "01-special.gb";
    let rom_path = PathBuf::from(INDV_ROM_PATH).join(test_name);
    assert!(rom_path.exists(), "Test ROM {rom_path:?} not found!");

    let out_log_path = PathBuf::from("out")
        .join(INDV_ROM_PATH)
        .join(format!("{test_name}.log"));

    let rom = fs::read(rom_path).expect("Failed to read ROM");
    let mut emu = Emulator::new()
        .with_cartridge(&rom)
        .with_serial_output_file(out_log_path.to_str().unwrap())
        .with_serial_output_buffer();

    let status = run_emulator(&mut emu);

    let output_buffer = emu.get_serial_output_buffer().unwrap();
    assert!(
        status.is_success(),
        "Output buffer:\n{}",
        String::from_utf8_lossy(output_buffer)
    );
}

#[test]
fn test_blargg_cpu_instrs_02_interrupts() {
    yagber::init_tracing();

    let test_name = "02-interrupts.gb";
    let rom_path = PathBuf::from(INDV_ROM_PATH).join(test_name);
    assert!(rom_path.exists(), "Test ROM {rom_path:?} not found!");

    let out_log_path = PathBuf::from("out")
        .join(INDV_ROM_PATH)
        .join(format!("{test_name}.log"));

    let rom = fs::read(rom_path).expect("Failed to read ROM");
    let mut emu = Emulator::new()
        .with_cartridge(&rom)
        .with_serial_output_file(out_log_path.to_str().unwrap())
        .with_serial_output_buffer();

    let status = run_emulator(&mut emu);

    let output_buffer = emu.get_serial_output_buffer().unwrap();
    assert!(
        status.is_success(),
        "Output buffer:\n{}",
        String::from_utf8_lossy(output_buffer)
    );
}
