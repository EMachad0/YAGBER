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
        let frames = data.len() / 2;
        for i in 0..frames {
            let l = left_buffer.try_pop().unwrap_or(0.0);
            let r = right_buffer.try_pop().unwrap_or(0.0);
            data[i * 2] = l;
            data[i * 2 + 1] = r;
        }
    }

    fn error_callback(_err: cpal::StreamError) {
        #[cfg(feature = "trace")]
        tracing::error!("error while playing stream: {}", _err);
    }
}

impl yagber_app::Component for OutputStream {}
