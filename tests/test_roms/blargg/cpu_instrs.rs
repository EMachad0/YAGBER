use std::{fs, path::PathBuf};

use crate::blargg::run_emulator;

const ROM_PATH: &str = "test_roms/blargg/cpu_instrs/cpu_instrs.gb";
const INDV_ROM_PATH: &str = "test_roms/blargg/cpu_instrs/individual/";

#[test]
fn test_blargg_cpu_instrs() {
    assert!(fs::metadata(ROM_PATH).is_ok(), "Test ROM not found!");

    let out_log_path = format!("out/{ROM_PATH}.log");

    let rom = fs::read(ROM_PATH).expect("Failed to read ROM");

    let status = run_emulator(&rom, &out_log_path);
    let is_ok = status.is_ok();
    if let Err((error, output_buffer)) = status {
        println!("Error: {error:?}");
        println!("Output buffer:\n{output_buffer}");
    }
    assert!(is_ok);
}

fn cpu_instrs_individual_test(test_name: &str) {
    let rom_path = PathBuf::from(INDV_ROM_PATH).join(test_name);
    assert!(rom_path.exists(), "Test ROM {rom_path:?} not found!");

    let out_log_path = PathBuf::from("out")
        .join(INDV_ROM_PATH)
        .join(format!("{test_name}.log"));

    let rom = fs::read(rom_path).expect("Failed to read ROM");

    let status = run_emulator(&rom, out_log_path.to_str().unwrap());

    let is_ok = status.is_ok();
    if let Err((error, output_buffer)) = status {
        println!("Error: {error:?}");
        println!("Output buffer:\n{output_buffer}");
    }
    assert!(is_ok);
}

#[test]
fn test_blargg_cpu_instrs_01_special() {
    cpu_instrs_individual_test("01-special.gb");
}

#[test]
fn test_blargg_cpu_instrs_02_interrupts() {
    cpu_instrs_individual_test("02-interrupts.gb");
}

#[test]
fn test_blargg_cpu_instrs_03_op_sp_hl() {
    cpu_instrs_individual_test("03-op sp,hl.gb");
}

#[test]
fn test_blargg_cpu_instrs_04_op_r_imm() {
    cpu_instrs_individual_test("04-op r,imm.gb");
}

#[test]
fn test_blargg_cpu_instrs_05_op_rp() {
    cpu_instrs_individual_test("05-op rp.gb");
}

#[test]
fn test_blargg_cpu_instrs_06_ld_r_r() {
    cpu_instrs_individual_test("06-ld r,r.gb");
}

#[test]
fn test_blargg_cpu_instrs_07_jr_jp_call_ret_rst() {
    cpu_instrs_individual_test("07-jr,jp,call,ret,rst.gb");
}

#[test]
fn test_blargg_cpu_instrs_08_misc_instrs() {
    cpu_instrs_individual_test("08-misc instrs.gb");
}

#[test]
fn test_blargg_cpu_instrs_09_op_r_r() {
    cpu_instrs_individual_test("09-op r,r.gb");
}

#[test]
fn test_blargg_cpu_instrs_10_bit_ops() {
    cpu_instrs_individual_test("10-bit ops.gb");
}

#[test]
fn test_blargg_cpu_instrs_11_op_a_hl_() {
    cpu_instrs_individual_test("11-op a,(hl).gb");
}
