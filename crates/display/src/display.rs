use winit::window::Window;

pub struct Display {
    pub window: Window,
}

impl Display {
    pub fn new(window: Window) -> Self {
        Self { window }
    }
}
