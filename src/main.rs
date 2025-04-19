use yagber::Emulator;

fn main() {
    yagber::init_tracing();

    Emulator::new().with_boot_rom().run();
}
