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
        let left_buffer = apu.left_buffer.clone();
        let right_buffer = apu.right_buffer.clone();
        #[cfg(feature = "trace")]
        tracing::trace!("Building output stream");
        let stream = device
            .build_output_stream(
                &config,
                move |data, info| Self::data_callback(&left_buffer, &right_buffer, data, info),
                Self::error_callback,
                Self::TIMEOUT,
            )
            .expect("failed to build output stream");

        stream.play().expect("failed to play stream");

        Self { _stream: stream }
    }

    fn data_callback(
        left_buffer: &yagber_apu::AudioBuffer,
        right_buffer: &yagber_apu::AudioBuffer,
        data: &mut [f32],
        _: &cpal::OutputCallbackInfo,
    ) {
        data.fill(0.0);
        let data_points = data.len() / 2;
        let left_samples = left_buffer.drain();
        let right_samples = right_buffer.drain();
        if left_samples.len() + right_samples.len() < data.len() {
            for (i, (left, right)) in left_samples.iter().zip(right_samples.iter()).enumerate() {
                data[i * 2] = *left;
                data[i * 2 + 1] = *right;
            }
        } else {
            let chunk_low = left_samples.len() / data_points;
            let high_count = left_samples.len() % data_points;
            let mut iter = left_samples.iter().zip(right_samples.iter());
            for i in 0..data_points {
                let mut left_sum = 0.0;
                let mut right_sum = 0.0;
                let chunk_len = chunk_low + (i < high_count) as usize;
                for _ in 0..chunk_len {
                    let (left, right) = iter.next().unwrap();
                    left_sum += left;
                    right_sum += right;
                }
                data[i * 2] = left_sum / chunk_len as f32;
                data[i * 2 + 1] = right_sum / chunk_len as f32;
            }
        }
    }

    fn error_callback(_err: cpal::StreamError) {
        #[cfg(feature = "trace")]
        tracing::error!("error while playing stream: {}", _err);
    }
}

impl yagber_app::Component for OutputStream {}
