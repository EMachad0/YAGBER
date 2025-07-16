mod acceptance;

use crate::utils::{MAX_FRAMES, TestError};

const EXPECTED_SUCCESS: &[u8] = &[3, 5, 8, 13, 21, 34];

/// Run the emulator for a specified number of frames
/// and check the output buffer for "Passed" or "Failed"
pub fn run_emulator(
    rom: &[u8],
    out_log_path: &str,
) -> <MtsTestRunner as yagber_app::Runner>::Result {
    yagber::Emulator::new()
        // Log must be first
        .with_plugin(yagber_log::LogPlugin::default())
        // Memory must be second
        .with_plugin(yagber_memory::MemoryPlugin::default().with_cartridge(rom))
        .with_plugin(yagber_cpu::CpuPlugin)
        .with_plugin(yagber_ppu::PpuPlugin)
        .with_plugin(yagber_dma::DmaPlugin)
        .with_plugin(
            yagber_link_cable::LinkCablePlugin::default()
                .with_serial_output_buffer()
                .with_serial_output_file(out_log_path),
        )
        // Timer must be last
        .with_plugin(yagber_timer::TimerPlugin)
        .run::<MtsTestRunner>()
}

pub struct MtsTestRunner {
    emulator: yagber::Emulator,
}

impl MtsTestRunner {
    pub fn new(emulator: yagber::Emulator) -> Self {
        Self { emulator }
    }

    pub fn run_until_result(&mut self) -> <MtsTestRunner as yagber_app::Runner>::Result {
        for _ in 0..MAX_FRAMES {
            self.emulator.step();
            let Some(buf) = yagber_link_cable::LinkCable::output_buffer_for(&mut self.emulator)
            else {
                panic!("No output buffer");
            };

            if buf.len() == EXPECTED_SUCCESS.len() {
                if buf == EXPECTED_SUCCESS {
                    return Ok(());
                } else {
                    let output_buffer = String::from_utf8_lossy(buf).to_string();
                    return Err((TestError::Failed, output_buffer));
                }
            }
        }
        Err((TestError::TimedOut, String::new()))
    }
}

impl yagber::app::Runner for MtsTestRunner {
    type Result = Result<(), (TestError, String)>;

    fn new(emulator: yagber::Emulator) -> Self {
        Self::new(emulator)
    }

    fn run(mut self) -> Self::Result {
        self.run_until_result()
    }
}
