use yagber::Emulator;

fn main() {
    yagber::init_tracing();

    Emulator::new().run();
}
