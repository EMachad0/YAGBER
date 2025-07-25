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
        #[cfg(feature = "trace-span")]
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
        #[cfg(feature = "trace-span")]
        let _span = tracing::info_span!("winit app window event").entered();

        use winit::event::WindowEvent;

        match event {
            WindowEvent::CloseRequested => {
                #[cfg(feature = "trace")]
                tracing::info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                #[cfg(feature = "trace-span")]
                let _span = tracing::info_span!("winit app redraw requested").entered();

                if !self.emulator.has_component::<Display>() {
                    return;
                }
                let (display, ppu) = self
                    .emulator
                    .get_components_mut2::<Display, yagber_ppu::Ppu>()
                    .expect("Display and/or PPU component missing");

                let frame_buffer = ppu.frame_buffer();
                for (i, pixel) in display.frame_buffer().chunks_exact_mut(4).enumerate() {
                    pixel.copy_from_slice(&frame_buffer[i]);
                }
                display.render().unwrap();

                #[cfg(feature = "trace")]
                tracing::trace!("Redraw requested");
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let Some(input_state) = self
                    .emulator
                    .get_component_mut::<yagber_input::InputState>()
                else {
                    return;
                };

                let keyboard_input = crate::input_converter::convert_keyboard_input(&event);
                let Some(input) = yagber_input::Input::from_keyboard_input(&keyboard_input) else {
                    return;
                };
                input_state.handle_input(input);
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        #[cfg(feature = "trace-span")]
        let _span = tracing::info_span!("winit app about to wait").entered();

        #[cfg(feature = "frame_marker")]
        tracing::event!(
            tracing::Level::INFO,
            message = "frame_marker",
            frame_marker = true
        );

        for _ in 0..yagber_ppu::Ppu::DOTS_PER_FRAME {
            self.emulator.step();

            let ppu = self
                .emulator
                .get_component::<yagber_ppu::Ppu>()
                .expect("PPU component missing");
            if ppu.just_entered_mode(yagber_ppu::PpuMode::VBlank) {
                self.emulator
                    .get_component_mut::<Display>()
                    .unwrap()
                    .request_redraw();
                // Break out of the loop so it doesn't start rendering the next frame.
                break;
            }
        }
    }
}
