use strum::EnumCount;

use crate::{
    input::{InputEvent, InputKey},
    key_state::KeyState, InputEventQueue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
pub enum JoypKey {
    ButtonA,
    ButtonB,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

impl JoypKey {
    pub fn from_input_key(input_key: InputKey) -> Option<Self> {
        match input_key {
            InputKey::ButtonA => Some(JoypKey::ButtonA),
            InputKey::ButtonB => Some(JoypKey::ButtonB),
            InputKey::Select => Some(JoypKey::Select),
            InputKey::Start => Some(JoypKey::Start),
            InputKey::Up => Some(JoypKey::Up),
            InputKey::Down => Some(JoypKey::Down),
            InputKey::Left => Some(JoypKey::Left),
            InputKey::Right => Some(JoypKey::Right),
            // _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JoypInputState {
    key_states: [KeyState; JoypKey::COUNT],
}

impl JoypInputState {
    pub fn new() -> Self {
        Self {
            key_states: [KeyState::Released; JoypKey::COUNT],
        }
    }

    pub(crate) fn on_tcycle(emulator: &mut yagber_app::Emulator) {
        let (event_queue, joyp_input_state) = emulator
            .get_components_mut2::<InputEventQueue, JoypInputState>()
            .expect("JoypInputState and InputEventQueue must be initialized");
        while let Some(event) = event_queue.pop_event::<Self>() {
            // #[cfg(feature = "trace")]
            // tracing::debug!("joyp_input: {event:?}");
            joyp_input_state.handle_input(event);
        }
    }

    fn handle_input(&mut self, input: InputEvent) {
        #[cfg(feature = "trace")]
        tracing::trace!("Joyp Input: {:?}", input);

        let joyp_key = JoypKey::from_input_key(input.input_key);
        let Some(joyp_key) = joyp_key else {
            return;
        };

        *self.key_state_mut(joyp_key) = input.state;
    }

    fn key_state(&self, key: JoypKey) -> &KeyState {
        &self.key_states[key as usize]
    }

    fn key_state_mut(&mut self, key: JoypKey) -> &mut KeyState {
        &mut self.key_states[key as usize]
    }

    pub(crate) fn joyp_transformer(&mut self, (_old_value, new_value): (u8, u8)) -> Option<u8> {
        let joyp = yagber_memory::JoypRegister::new(new_value);
        let selected_buttons = joyp.selected_buttons();
        let lower_nibble = self.lower_nibble(selected_buttons);
        let new_joyp = 0xC0 | selected_buttons.as_bits() | lower_nibble;
        Some(new_joyp)
    }

    fn lower_nibble(&self, selected_buttons: yagber_memory::SelectedButtons) -> u8 {
        match selected_buttons {
            yagber_memory::SelectedButtons::None => 0x0F,
            yagber_memory::SelectedButtons::Buttons => {
                let mut lower_nibble = 0x00;
                if !self.key_state(JoypKey::ButtonA).is_pressed() {
                    lower_nibble |= 0x01;
                }
                if !self.key_state(JoypKey::ButtonB).is_pressed() {
                    lower_nibble |= 0x02;
                }
                if !self.key_state(JoypKey::Select).is_pressed() {
                    lower_nibble |= 0x04;
                }
                if !self.key_state(JoypKey::Start).is_pressed() {
                    lower_nibble |= 0x08;
                }
                lower_nibble
            }
            yagber_memory::SelectedButtons::Directions => {
                let mut lower_nibble = 0x00;
                if !self.key_state(JoypKey::Right).is_pressed() {
                    lower_nibble |= 0x01;
                }
                if !self.key_state(JoypKey::Left).is_pressed() {
                    lower_nibble |= 0x02;
                }
                if !self.key_state(JoypKey::Up).is_pressed() {
                    lower_nibble |= 0x04;
                }
                if !self.key_state(JoypKey::Down).is_pressed() {
                    lower_nibble |= 0x08;
                }
                lower_nibble
            }
            yagber_memory::SelectedButtons::Both => 0x0F,
        }
    }
}

impl Default for JoypInputState {
    fn default() -> Self {
        Self::new()
    }
}

impl yagber_app::Component for JoypInputState {}
