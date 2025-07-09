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
            .with_inner_size(LogicalSize::new(
                Display::WIDTH * Display::SCALE_FACTOR,
                Display::HEIGHT * Display::SCALE_FACTOR,
            ))
            .with_resizable(false)
            .with_title("YAGBER")
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("winit app resumed").entered();

        let display = self.emulator.get_component::<Display>();
        if display.is_none() {
            let window = event_loop
                .create_window(Self::window_attributes())
                .expect("Failed to create window");
            let display = Display::new(window).expect("Failed to create display");
            self.emulator.with_component(display);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("winit app window event").entered();

        use winit::event::WindowEvent;

        match event {
            WindowEvent::CloseRequested => {
                #[cfg(feature = "trace")]
                tracing::info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                #[cfg(feature = "trace")]
                let _span = tracing::info_span!("winit app redraw requested").entered();

                let display = self.emulator.get_component_mut::<Display>();
                if display.is_none() {
                    return;
                }

                let display = display.unwrap();
                display.render().unwrap();

                #[cfg(feature = "trace")]
                tracing::trace!("Redraw requested");
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("winit app about to wait").entered();

        #[cfg(feature = "frame_marker")]
        tracing::event!(
            tracing::Level::INFO,
            message = "frame_marker",
            frame_marker = true
        );

        for _ in 0..72224 {
            self.emulator.step();
        }

        // if self.emulator.frame_ready() {
        //     if let Some(display) = self.emulator.get_component::<Display>() {
        //         display.window.request_redraw();
        //     }
        // }
    }
}
