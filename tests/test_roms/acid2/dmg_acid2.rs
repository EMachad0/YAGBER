use std::fs;

use crate::acid2::run_emulator;

#[test]
fn test_dmg_acid2() {
    let rom_path = "test_roms/dmg-acid2/dmg-acid2.gb";
    let expected_screen_path = "test_roms/dmg-acid2/img/reference-cgb.png";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");
    assert!(
        fs::metadata(expected_screen_path).is_ok(),
        "Expected screen not found!"
    );

    let out_log_path = format!("out/{rom_path}.log");
    let out_screen_path = format!("out/{rom_path}.png");

    let rom = fs::read(rom_path).expect("Failed to read ROM");

    let status = run_emulator(&rom, &out_log_path, expected_screen_path);

    let is_ok = status.is_ok();
    if let Err((error, output_screen)) = status {
        println!("Error: {error:?}");
        crate::acid2::save_screen(&output_screen, &out_screen_path);
    }

    assert!(is_ok);
}
