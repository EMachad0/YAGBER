use cpal::traits::{DeviceTrait, StreamTrait};

pub struct OutputStream {
    _stream: cpal::Stream,
}

impl OutputStream {
    const TIMEOUT: Option<std::time::Duration> = None; // None=blocking, Some(Duration)=timeout

    pub fn new(
        device: cpal::Device,
        config: cpal::StreamConfig,
        apu: &mut yagber_apu::Apu,
    ) -> Self {
        let buffer = apu.buffer.clone();
        #[cfg(feature = "trace")]
        tracing::trace!("Building output stream");
        let stream = device
            .build_output_stream(
                &config,
                move |data, info| Self::data_callback(&buffer, data, info),
                Self::error_callback,
                Self::TIMEOUT,
            )
            .expect("failed to build output stream");

        stream.play().expect("failed to play stream");

        Self { _stream: stream }
    }

    fn data_callback(
        buffer: &yagber_apu::AudioBuffer,
        data: &mut [f32],
        _: &cpal::OutputCallbackInfo,
    ) {
        let buffer = buffer.lock().drain(..).collect::<Vec<_>>();
        if buffer.len() < data.len() {
            return;
        }
        let data_points = data.len() / 2;
        let left_samples = buffer
            .iter()
            .enumerate()
            .filter_map(|(i, sample)| if i % 2 == 0 { Some(sample) } else { None })
            .collect::<Vec<_>>();
        let right_samples = buffer
            .iter()
            .enumerate()
            .filter_map(|(i, sample)| if i % 2 == 1 { Some(sample) } else { None })
            .collect::<Vec<_>>();
        let samples_per_datapoint = buffer.len() / data_points;
        for (i, (left, right)) in left_samples
            .chunks(samples_per_datapoint)
            .zip(right_samples.chunks(samples_per_datapoint))
            .enumerate()
        {
            let left_sum = left.iter().fold(0.0, |acc, x| acc + *x);
            let right_sum = right.iter().fold(0.0, |acc, x| acc + *x);
            data[i * 2] = left_sum / samples_per_datapoint as f32;
            data[i * 2 + 1] = right_sum / samples_per_datapoint as f32;
        }
    }

    fn error_callback(_err: cpal::StreamError) {
        #[cfg(feature = "trace")]
        tracing::error!("error while playing stream: {}", _err);
    }
}

impl yagber_app::Component for OutputStream {}
