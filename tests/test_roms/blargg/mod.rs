mod cpu_instrs;
mod halt_bug;

use crate::utils::{MAX_CYCLES, TestResult};

pub fn run_emulator(rom: &[u8], out_log_path: &str) -> BlarggTestRunnerResult {
    // Order matters, some plugins depend on others
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
        .run::<BlarggTestRunner>()
}

pub struct BlarggTestRunnerResult {
    result: TestResult,
    output_buffer: String,
}

pub struct BlarggTestRunner {
    emulator: yagber::Emulator,
}

impl BlarggTestRunner {
    pub fn new(emulator: yagber::Emulator) -> Self {
        Self { emulator }
    }

    /// Run the emulator for a specified number of cycles
    /// and check the output buffer for "Passed" or "Failed"
    fn run_until_result(&mut self) -> TestResult {
        for _ in 0..MAX_CYCLES {
            self.emulator.step();
            let Some(buf) = yagber_link_cable::LinkCable::output_buffer_for(&mut self.emulator)
            else {
                panic!("No output buffer");
            };

            let buf = String::from_utf8_lossy(buf);
            if buf.contains("Passed") {
                return TestResult::Passed;
            } else if buf.contains("Failed") {
                return TestResult::Failed;
            }
        }
        TestResult::TimedOut
    }
}

impl yagber::app::Runner for BlarggTestRunner {
    type Result = BlarggTestRunnerResult;

    fn new(emulator: yagber::Emulator) -> Self {
        Self::new(emulator)
    }

    fn run(&mut self) -> Self::Result {
        let result = self.run_until_result();
        let output_buffer =
            yagber_link_cable::LinkCable::output_buffer_for(&mut self.emulator).unwrap();
        BlarggTestRunnerResult {
            result,
            output_buffer: String::from_utf8_lossy(output_buffer).to_string(),
        }
    }
}
