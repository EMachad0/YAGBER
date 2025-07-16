pub const MAX_FRAMES: u32 = 60 * 60 * 60; // frames

#[derive(Debug, PartialEq, Eq)]
pub enum TestError {
    Failed,
    TimedOut,
}

pub fn run_boot(emulator: &mut yagber::Emulator) -> Result<u8, TestError> {
    // Run the boot sequence
    for _ in 0..MAX_FRAMES {
        let memory_bus = emulator
            .get_component::<yagber_memory::Bus>()
            .expect("Memory bus not found");
        let boot_reg = memory_bus
            .io_registers
            .read(yagber_memory::IOType::BANK.address());
        if boot_reg == 0x11 {
            return Ok(boot_reg);
        }

        emulator.step();
    }
    Err(TestError::TimedOut)
}
