mod acceptance;

use crate::utils::{MAX_CYCLES, TestResult};

const EXPECTED_SUCCESS: &[u8] = &[3, 5, 8, 13, 21, 34];

/// Run the emulator for a specified number of cycles
/// and check the output buffer for "Passed" or "Failed"
pub fn run_emulator(rom: &[u8], out_log_path: &str) -> MtsTestRunnerResult {
    yagber::Emulator::new()
        // Memory must be first
        .with_plugin(yagber_memory::MemoryPlugin::default().with_cartridge(rom))
        .with_plugin(yagber_cpu::CpuPlugin)
        .with_plugin(yagber_ppu::PpuPlugin)
        .with_plugin(
            yagber_link_cable::LinkCablePlugin::default()
                .with_serial_output_buffer()
                .with_serial_output_file(out_log_path),
        )
        // Timer must be last
        .with_plugin(yagber_timer::TimerPlugin)
        .run::<MtsTestRunner>()
}

pub struct MtsTestRunnerResult {
    result: TestResult,
    output_buffer: String,
}

pub struct MtsTestRunner {
    emulator: yagber::Emulator,
}

impl MtsTestRunner {
    pub fn new(emulator: yagber::Emulator) -> Self {
        Self { emulator }
    }

    pub fn run_until_result(&mut self) -> TestResult {
        for _ in 0..MAX_CYCLES {
            self.emulator.step();
            let Some(buf) = yagber_link_cable::LinkCable::output_buffer_for(&mut self.emulator)
            else {
                panic!("No output buffer");
            };

            if buf.len() == EXPECTED_SUCCESS.len() {
                if buf == EXPECTED_SUCCESS {
                    return TestResult::Passed;
                } else {
                    return TestResult::Failed;
                }
            }
        }
        TestResult::TimedOut
    }
}

impl yagber::app::Runner for MtsTestRunner {
    type Result = MtsTestRunnerResult;

    fn new(emulator: yagber::Emulator) -> Self {
        Self::new(emulator)
    }

    fn run(&mut self) -> Self::Result {
        let result = self.run_until_result();
        let output_buffer =
            yagber_link_cable::LinkCable::output_buffer_for(&mut self.emulator).unwrap();
        MtsTestRunnerResult {
            result,
            output_buffer: String::from_utf8_lossy(output_buffer).to_string(),
        }
    }
}
