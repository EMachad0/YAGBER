use std::fs;

use crate::{acid2::run_emulator, utils::TestResult};
use pretty_assertions::assert_eq;

#[test]
fn test_cgb_acid2() {
    let rom_path = "test_roms/cgb-acid2/cgb-acid2.gbc";
    let expected_screen_path = "test_roms/cgb-acid2/img/reference.png";
    assert!(fs::metadata(rom_path).is_ok(), "Test ROM not found!");
    assert!(
        fs::metadata(expected_screen_path).is_ok(),
        "Expected screen not found!"
    );

    let out_log_path = format!("out/{}.log", rom_path);
    let out_screen_path = format!("out/{}.png", rom_path);

    let rom = fs::read(rom_path).expect("Failed to read ROM");

    let status = run_emulator(&rom, &out_log_path, expected_screen_path);

    if !status.result.is_success() {
        crate::acid2::save_screen(&status.output_screen, &out_screen_path);
    }

    assert_eq!(status.result, TestResult::Passed);
}
