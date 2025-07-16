/// The current "press" state of an element
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum KeyState {
    /// The button is pressed.
    Pressed,
    /// The button is not pressed.
    Released,
}

impl KeyState {
    /// Is this button pressed?
    pub fn is_pressed(&self) -> bool {
        matches!(self, KeyState::Pressed)
    }
}
