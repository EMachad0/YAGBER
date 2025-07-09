use winit::event_loop::EventLoop;
use yagber_app::Emulator;

use crate::winit_app::WinitApp;

pub struct WinitRunner {
    event_loop: EventLoop<()>,
    emulator: Emulator,
}

impl yagber_app::Runner for WinitRunner {
    type Result = ();

    fn new(emulator: Emulator) -> Self {
        let event_loop = EventLoop::new().unwrap();
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        Self {
            emulator,
            event_loop,
        }
    }

    fn run(self) -> Self::Result {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("winit runner run").entered();
        let mut app = WinitApp::new(self.emulator);
        let result = self.event_loop.run_app(&mut app);

        if let Some(error) = result.err() {
            #[cfg(feature = "trace")]
            tracing::error!("Error running event loop: {:?}", error);
        }
    }
}
