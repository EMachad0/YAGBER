use std::fs;

// This test is not run currently because it requires a screen
#[allow(dead_code)]
fn test_little_things_gb_double_halt_cancel() {
    let rom_path = "test_roms/little-things-gb/double-halt-cancel-gbconly.gb";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");
}
