pub const MAX_CYCLES: u32 = 60 * 60 * 60; // frames

#[derive(Debug, PartialEq, Eq)]
pub enum TestResult {
    Passed,
    Failed,
    TimedOut,
}

impl TestResult {
    pub fn is_success(&self) -> bool {
        self == &TestResult::Passed
    }
}

pub fn run_boot(emulator: &mut yagber::Emulator) -> Result<(), TestResult> {
    // Run the boot sequence
    for _ in 0..MAX_CYCLES {
        let bus_ptr = emulator
            .get_component::<yagber_memory::Bus>()
            .expect("Bus not found");
        let boot_reg = bus_ptr
            .io_registers
            .read(yagber_memory::IOType::BANK.address());
        println!("boot_reg: {:#02X}", boot_reg);
        if boot_reg == 0x11 {
            return Ok(());
        }

        emulator.step();
    }
    Err(TestResult::TimedOut)
}
