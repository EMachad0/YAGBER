use crate::utils::{TestResult, run_boot};

mod cgb_acid2;
mod dmg_acid2;

pub fn run_emulator(
    rom: &[u8],
    out_log_path: &str,
    expected_screen_path: &str,
) -> Acid2TestRunnerResult {
    let expected_screen = ExpectedScreen::from_file(expected_screen_path);

    // Order matters, some plugins depend on others
    let mut emulator = yagber::Emulator::new();
    emulator.with_component(expected_screen);
    emulator
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
        .run::<Acid2TestRunner>()
}

pub struct ExpectedScreen {
    bytes: Vec<u8>,
}

impl ExpectedScreen {
    pub fn from_file(file_path: &str) -> Self {
        let screen = image::open(file_path).expect("Failed to read file");
        let screen = screen.to_rgba8();
        let screen = screen.into_raw();
        Self { bytes: screen }
    }

    pub fn screen(&self) -> &[u8] {
        &self.bytes
    }
}

impl yagber_app::Component for ExpectedScreen {}

pub struct Acid2TestRunnerResult {
    result: TestResult,
    output_screen: Vec<u8>,
}

pub struct Acid2TestRunner {
    emulator: yagber::Emulator,
    expected_screen: Vec<u8>,
}

impl Acid2TestRunner {
    pub fn new(emulator: yagber::Emulator) -> Self {
        let expected_screen = emulator
            .get_component::<ExpectedScreen>()
            .expect("ExpectedScreen not found")
            .screen()
            .to_vec();
        Self {
            emulator,
            expected_screen,
        }
    }

    fn run_until_result(&mut self) -> (TestResult, Vec<u8>) {
        let boot_res = run_boot(&mut self.emulator);
        if let Err(result) = boot_res {
            return (result, Vec::new());
        }

        for _ in 0..15 {
            self.emulator.step();
        }

        let output_screen = self
            .emulator
            .get_component::<yagber_ppu::Ppu>()
            .expect("Display not found")
            .frame_buffer()
            .as_flattened()
            .to_vec();

        let test_result = if output_screen == self.expected_screen {
            TestResult::Passed
        } else {
            TestResult::Failed
        };

        (test_result, output_screen)
    }
}

impl yagber_app::Runner for Acid2TestRunner {
    type Result = Acid2TestRunnerResult;

    fn new(emulator: yagber::Emulator) -> Self {
        Self::new(emulator)
    }

    fn run(mut self) -> Self::Result {
        let (result, output_screen) = self.run_until_result();
        Acid2TestRunnerResult {
            result,
            output_screen,
        }
    }
}

pub fn save_screen(screen: &[u8], path: &str) {
    use image::codecs::png::PngEncoder;
    let mut png_bytes = Vec::new();
    {
        let encoder = PngEncoder::new(&mut png_bytes);
        use image::ImageEncoder as _;
        encoder
            .write_image(
                screen,
                yagber_display::Display::WIDTH,
                yagber_display::Display::HEIGHT,
                image::ExtendedColorType::Rgba8,
            )
            .expect("Failed to encode PNG");
    }
    let screen = image::load_from_memory(&png_bytes).unwrap();
    screen.save(path).unwrap();
}
