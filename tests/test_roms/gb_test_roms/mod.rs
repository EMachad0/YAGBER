mod cpu_instrs;

use crate::utils::{MAX_CYCLES, TestResult};

/// Run the emulator for a specified number of cycles
/// and check the output buffer for "Passed" or "Failed"
pub fn run_emulator(emu: &mut yagber::Emulator) -> TestResult {
    for _ in 0..MAX_CYCLES {
        emu.step();
        if let Some(buf) = emu.get_serial_output_buffer() {
            let buf = String::from_utf8_lossy(buf);
            if buf.contains("Passed") {
                return TestResult::Passed;
            } else if buf.contains("Failed") {
                return TestResult::Failed;
            }
        }
    }
    TestResult::TimedOut
}
