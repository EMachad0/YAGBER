use cpal::traits::{DeviceTrait, StreamTrait};
use ringbuf::traits::Consumer;

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
        let mut left_buffer: yagber_apu::ConsumerCache =
            apu.left_buffer.take_consumer().expect("No left buffer");
        let mut right_buffer: yagber_apu::ConsumerCache =
            apu.right_buffer.take_consumer().expect("No right buffer");
        #[cfg(feature = "trace")]
        tracing::trace!("Building output stream");
        let stream = device
            .build_output_stream(
                &config,
                move |data, info| {
                    Self::data_callback(&mut left_buffer, &mut right_buffer, data, info)
                },
                Self::error_callback,
                Self::TIMEOUT,
            )
            .expect("failed to build output stream");

        stream.play().expect("failed to play stream");

        Self { _stream: stream }
    }

    fn data_callback(
        left_buffer: &mut yagber_apu::ConsumerCache,
        right_buffer: &mut yagber_apu::ConsumerCache,
        data: &mut [f32],
        _: &cpal::OutputCallbackInfo,
    ) {
        data.fill(0.0);
        let data_points = data.len() / 2;
        let right_samples = right_buffer.pop_iter().collect::<Vec<_>>();
        let left_samples = left_buffer
            .pop_iter()
            .take(right_samples.len())
            .collect::<Vec<_>>();
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
