mod output_stream;

use cpal::traits::{DeviceTrait, HostTrait};

pub struct CpalPlugin;

impl yagber_app::Plugin for CpalPlugin {
    fn init(self, emulator: &mut yagber_app::Emulator) {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        let config = supported_config.config();

        let stream = output_stream::OutputStream::new(device, config);

        emulator.with_component(stream);
    }
}
