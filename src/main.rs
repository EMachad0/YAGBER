use yagber::Emulator;
use yagber::HeadlessRunner;

fn main() {
    yagber::init_tracing();

    Emulator::new().run::<HeadlessRunner>();
}
