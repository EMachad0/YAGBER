mod input;
mod input_state;
mod key_state;
mod physical_input;

pub use input::{Input, InputKey};
pub use input_state::InputState;
pub use key_state::KeyState;
pub use physical_input::keyboard;

pub struct InputPlugin;

impl yagber_app::Plugin for InputPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        emulator.with_component(input_state::InputState::new());

        let joyp_transformer = emulator.attach_component(input_state::InputState::joyp_transformer);

        emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("InputPlugin must be initialized after MemoryPlugin")
            .io_registers
            .add_transformer(yagber_memory::IOType::JOYP, joyp_transformer);
    }
}
