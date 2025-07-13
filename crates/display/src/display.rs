use winit::window::Window;

pub struct Display {
    window: std::sync::Arc<Window>,
    pixels: pixels::Pixels<'static>,
}

impl Display {
    pub const WIDTH: u32 = 256;
    pub const HEIGHT: u32 = 256;
    pub const SCALE_FACTOR: u32 = 4;

    pub fn new(window: Window) -> Result<Self, pixels::Error> {
        let window = std::sync::Arc::new(window);
        let window_size = window.inner_size();
        let surface_texture =
            pixels::SurfaceTexture::new(window_size.width, window_size.height, window.clone());
        let pixels = pixels::Pixels::new(Self::WIDTH, Self::HEIGHT, surface_texture)?;

        Ok(Self { window, pixels })
    }

    pub fn frame_buffer(&mut self) -> &mut [u8] {
        self.pixels.frame_mut()
    }

    pub fn render(&mut self) -> Result<(), pixels::Error> {
        #[cfg(feature = "trace-span")]
        let _span = tracing::info_span!("display render").entered();

        self.pixels.render()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}

impl yagber_app::Component for Display {}
