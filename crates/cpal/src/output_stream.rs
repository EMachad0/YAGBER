use cpal::traits::{DeviceTrait, StreamTrait};

pub struct OutputStream {
    _stream: cpal::Stream,
}

impl OutputStream {
    const TIMEOUT: Option<std::time::Duration> = None; // None=blocking, Some(Duration)=timeout

    pub fn new(device: cpal::Device, config: cpal::StreamConfig) -> Self {
        #[cfg(feature = "trace")]
        tracing::trace!("Building output stream");
        let stream = device
            .build_output_stream(
                &config,
                Self::data_callback::<f32>,
                Self::error_callback,
                Self::TIMEOUT,
            )
            .expect("failed to build output stream");

        stream.play().expect("failed to play stream");

        Self { _stream: stream }
    }

    fn data_callback<T: cpal::Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
        for sample in data.iter_mut() {
            *sample = T::EQUILIBRIUM;
        }
    }

    fn error_callback(_err: cpal::StreamError) {
        #[cfg(feature = "trace")]
        tracing::error!("error while playing stream: {}", _err);
    }
}

impl yagber_app::Component for OutputStream {}
