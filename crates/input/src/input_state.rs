use strum::EnumCount;

use crate::{
    input::{Input, InputKey},
    key_state::KeyState,
};

#[derive(Debug, Clone, Copy)]
pub struct InputState {
    key_states: [KeyState; InputKey::COUNT],
}

impl InputState {
    pub fn new() -> Self {
        Self {
            key_states: [KeyState::Released; InputKey::COUNT],
        }
    }

    pub fn handle_input(&mut self, input: Input) {
        #[cfg(feature = "trace")]
        tracing::trace!("Input: {:?}", input);

        self.key_states[input.input_key as usize] = input.state;
    }

    fn key_state(&self, key: InputKey) -> &KeyState {
        &self.key_states[key as usize]
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
                if !self.key_state(InputKey::A).is_pressed() {
                    lower_nibble |= 0x01;
                }
                if !self.key_state(InputKey::B).is_pressed() {
                    lower_nibble |= 0x02;
                }
                if !self.key_state(InputKey::Select).is_pressed() {
                    lower_nibble |= 0x04;
                }
                if !self.key_state(InputKey::Start).is_pressed() {
                    lower_nibble |= 0x08;
                }
                lower_nibble
            }
            yagber_memory::SelectedButtons::Directions => {
                let mut lower_nibble = 0x00;
                if !self.key_state(InputKey::Right).is_pressed() {
                    lower_nibble |= 0x01;
                }
                if !self.key_state(InputKey::Left).is_pressed() {
                    lower_nibble |= 0x02;
                }
                if !self.key_state(InputKey::Up).is_pressed() {
                    lower_nibble |= 0x04;
                }
                if !self.key_state(InputKey::Down).is_pressed() {
                    lower_nibble |= 0x08;
                }
                lower_nibble
            }
            yagber_memory::SelectedButtons::Both => 0x0F,
        }
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl yagber_app::Component for InputState {}
