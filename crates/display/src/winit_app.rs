use winit::{application::ApplicationHandler, dpi::LogicalSize, window::WindowAttributes};
use yagber_app::Emulator;

use crate::display::Display;

pub struct WinitApp {
    emulator: Emulator,
}

impl WinitApp {
    pub fn new(emulator: Emulator) -> Self {
        Self { emulator }
    }

    pub fn window_attributes() -> WindowAttributes {
        WindowAttributes::default()
            .with_inner_size(LogicalSize::new(160, 144))
            .with_resizable(false)
            .with_title("YAGBER")
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let display = self.emulator.get_component::<Display>();
        if display.is_none() {
            let window = event_loop.create_window(Self::window_attributes()).unwrap();
            let display = Display::new(window);
            self.emulator.with_component(display);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use winit::event::WindowEvent;

        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Perform update
                self.emulator.step();

                let display = self.emulator.get_component::<Display>();
                if display.is_none() {
                    return;
                }

                let display = display.unwrap();
                display.window.request_redraw();
            }
            _ => (),
        }
    }
}
