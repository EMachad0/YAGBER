use crate::{key_state::KeyState, physical_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumCount)]
pub enum InputKey {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub input_key: InputKey,
    pub state: KeyState,
}

impl Input {
    pub fn new(input_key: InputKey, state: KeyState) -> Self {
        Self { input_key, state }
    }

    pub fn from_keyboard_input(input: &physical_input::keyboard::KeyboardInput) -> Option<Self> {
        use physical_input::keyboard::KeyCode;
        let input_key = match input.key_code {
            KeyCode::KeyZ => InputKey::A,
            KeyCode::KeyX => InputKey::B,
            KeyCode::Enter => InputKey::Start,
            KeyCode::Backspace => InputKey::Select,
            KeyCode::ArrowUp => InputKey::Up,
            KeyCode::ArrowDown => InputKey::Down,
            KeyCode::ArrowLeft => InputKey::Left,
            KeyCode::ArrowRight => InputKey::Right,
            _ => return None,
        };

        Some(Self::new(input_key, input.state))
    }
}
