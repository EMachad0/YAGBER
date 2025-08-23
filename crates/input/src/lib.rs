mod emulation_control;
mod input_event;
mod input_event_queue;
mod joyp_input_state;
mod key_state;
mod physical_input;

pub use input_event::InputEvent;
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
            .on_fixed_step(joyp_input_state::JoypInputState::on_mcycle)
            .on_fixed_step(emulation_control::EmulationControl::on_mcycle);

        emulator
            .get_component_mut::<input_event_queue::InputEventQueue>()
            .unwrap()
            .with_observer::<joyp_input_state::JoypInputState>()
            .with_observer::<emulation_control::EmulationControl>();

        let joyp_transformer =
            emulator.attach_component(joyp_input_state::JoypInputState::joyp_transformer);

        emulator
            .get_component_mut::<yagber_memory::Bus>()
            .expect("InputPlugin must be initialized after MemoryPlugin")
            .io_registers
            .add_transformer(yagber_memory::IOType::JOYP, joyp_transformer);
    }
}
