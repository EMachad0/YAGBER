mod dest;
mod link_cable;

pub use dest::Destination as LinkCableDest;
pub use link_cable::LinkCable;

pub struct LinkCablePlugin {
    link_cable: Option<LinkCable>,
}

impl LinkCablePlugin {
    pub fn new() -> Self {
        Self {
            link_cable: Some(LinkCable::new()),
        }
    }

    pub fn with_serial_output_file(mut self, path: &str) -> Self {
        self.link_cable.as_mut().unwrap().to_file(path);
        self
    }

    pub fn with_serial_output_buffer(mut self) -> Self {
        self.link_cable.as_mut().unwrap().to_buffer();
        self
    }

    pub fn with_serial_output_stdout(mut self) -> Self {
        self.link_cable.as_mut().unwrap().to_stdout();
        self
    }
}

impl yagber_app::Plugin for LinkCablePlugin {
    fn init(mut self, emulator: &mut yagber_app::Emulator) {
        let link_cable = std::mem::take(&mut self.link_cable).unwrap();
        emulator
            .with_component(link_cable)
            .with_event_handler(LinkCable::on_tcycle);
    }
}

impl Default for LinkCablePlugin {
    fn default() -> Self {
        Self::new()
    }
}
