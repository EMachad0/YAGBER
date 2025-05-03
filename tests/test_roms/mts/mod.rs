mod acceptance;

use crate::utils::{MAX_CYCLES, TestResult};

const EXPECTED_SUCESS: &[u8] = &[3, 5, 8, 13, 21, 34];

/// Run the emulator for a specified number of cycles
/// and check the output buffer for "Passed" or "Failed"
pub fn run_emulator(emu: &mut yagber::Emulator) -> TestResult {
    for _ in 0..MAX_CYCLES {
        emu.step();
        if let Some(buf) = emu.get_serial_output_buffer() {
            if buf.len() == EXPECTED_SUCESS.len() {
                if buf == EXPECTED_SUCESS {
                    return TestResult::Passed;
                } else {
                    return TestResult::Failed;
                }
            }
        }
    }
    TestResult::TimedOut
}
