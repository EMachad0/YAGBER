#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmulationControlEvent {
    Pause,
    Resume,
    TogglePause,
}
