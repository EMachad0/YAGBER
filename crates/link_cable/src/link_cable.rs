use yagber_memory::{Bus, Memory};

use crate::dest::{Destination, DestinationCollector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkCableMode {
    Slave,
    Master,
}

impl LinkCableMode {
    pub fn from_bit(bit: bool) -> Self {
        if bit { Self::Master } else { Self::Slave }
    }
}

#[derive(Debug, Default)]
pub struct LinkCable {
    destinations: DestinationCollector,
}

impl LinkCable {
    pub fn new() -> Self {
        Self {
            destinations: DestinationCollector::new(),
        }
    }

    pub fn to_buffer(&mut self) {
        self.destinations.add_dest(Destination::Buffer(Vec::new()));
    }

    pub fn to_file(&mut self, path: &str) {
        let path = std::path::Path::new(path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        let file = std::fs::File::create(path).unwrap_or_else(|_| {
            panic!("Failed to create file at path: {}", path.to_str().unwrap())
        });

        self.destinations.add_dest(Destination::File(file));
    }

    pub fn to_stdout(&mut self) {
        self.destinations
            .add_dest(Destination::Stdout(std::io::stdout()));
    }

    pub fn transfer_enabled(ram: &mut Bus) -> bool {
        ram.read_bit(yagber_memory::IOType::SC.address(), 7)
    }

    pub fn clock_speed(ram: &mut Bus) -> bool {
        ram.read_bit(yagber_memory::IOType::SC.address(), 1)
    }

    pub fn read_mode(ram: &mut Bus) -> LinkCableMode {
        let bit = ram.read_bit(yagber_memory::IOType::SC.address(), 0);
        LinkCableMode::from_bit(bit)
    }

    pub fn read_data(ram: &mut Bus) -> u8 {
        ram.read(yagber_memory::IOType::SB.address())
    }

    pub fn on_mcycle(emulator: &mut yagber_app::Emulator) {
        let (link_cable, bus) = emulator
            .get_components_mut2::<LinkCable, Bus>()
            .expect("LinkCable and/or Bus component missing");
        link_cable.step(bus);
    }

    pub fn step(&mut self, ram: &mut Bus) {
        if Self::transfer_enabled(ram) && Self::read_mode(ram) == LinkCableMode::Master {
            let data = Self::read_data(ram);
            let _result = self.destinations.write(data);
            #[cfg(feature = "trace")]
            if let Err(e) = _result {
                tracing::error!("Failed to write to destination: {}", e);
            }
            ram.write(yagber_memory::IOType::SB.address(), 0xFF);
            ram.write(yagber_memory::IOType::SC.address(), 0);
            ram.request_interrupt(yagber_memory::InterruptType::Serial);
        }
    }

    pub fn get_buffer(&self) -> Option<&[u8]> {
        self.destinations.get_buffer()
    }

    pub fn output_buffer_for(emulator: &mut yagber_app::Emulator) -> Option<&[u8]> {
        let link_cable = emulator.get_component_mut::<LinkCable>().unwrap();
        link_cable.get_buffer()
    }
}

impl yagber_app::Component for LinkCable {}
