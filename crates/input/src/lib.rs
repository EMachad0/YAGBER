mod input;
mod input_event_queue;
mod joyp_input_state;
mod key_state;
mod physical_input;

pub use input::{InputEvent, InputKey};
pub use input_event_queue::InputEventQueue;
pub use joyp_input_state::JoypInputState;
pub use key_state::KeyState;
pub use physical_input::keyboard;

pub struct InputPlugin;

impl yagber_app::Plugin for InputPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator
            .with_component(input_event_queue::InputEventQueue::default())
            .with_component(joyp_input_state::JoypInputState::new())
            .on_tcycle(joyp_input_state::JoypInputState::on_tcycle);

        emulator
            .get_component_mut::<input_event_queue::InputEventQueue>()
            .unwrap()
            .add_observer::<joyp_input_state::JoypInputState>();

        let joyp_transformer =
            emulator.attach_component(joyp_input_state::JoypInputState::joyp_transformer);

        emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("InputPlugin must be initialized after MemoryPlugin")
            .io_registers
            .add_transformer(yagber_memory::IOType::JOYP, joyp_transformer);
    }
}
