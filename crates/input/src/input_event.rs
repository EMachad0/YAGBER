use crate::physical_input;

#[derive(Debug, Clone)]
pub enum InputEvent {
    Keyboard(physical_input::keyboard::KeyboardInput),
}

impl From<physical_input::keyboard::KeyboardInput> for InputEvent {
    fn from(value: physical_input::keyboard::KeyboardInput) -> Self {
        Self::Keyboard(value)
    }
}
