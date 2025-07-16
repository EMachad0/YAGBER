use std::fs;

use crate::blargg::run_emulator;

const ROM_PATH: &str = "test_roms/blargg/halt_bug.gb";

// This test is not run currently because it does not have source code
#[allow(dead_code)]
fn test_halt_bug() {
    assert!(fs::metadata(ROM_PATH).is_ok(), "Test ROM not found!");

    let out_log_path = format!("out/{}.log", ROM_PATH);

    let rom = fs::read(ROM_PATH).expect("Failed to read ROM");
    let status = run_emulator(&rom, &out_log_path);
    let is_ok = status.is_ok();
    if let Err((error, output_buffer)) = status {
        println!("Error: {:?}", error);
        println!("Output buffer:\n{}", output_buffer);
    }
    assert!(is_ok);
}
