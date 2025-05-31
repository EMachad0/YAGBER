use std::fs;
use yagber::Emulator;

use crate::blargg::run_emulator;

const ROM_PATH: &str = "test_roms/blargg/halt_bug.gb";

// This test is not run currently because it does not have source code
#[allow(dead_code)]
fn test_halt_bug() {
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
