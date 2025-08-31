use ringbuf::{
    traits::{Producer, Split},
    wrap::caching::Caching,
};

pub type ProducerCache =
    Caching<std::sync::Arc<ringbuf::SharedRb<ringbuf::storage::Heap<f32>>>, true, false>;
pub type ConsumerCache =
    Caching<std::sync::Arc<ringbuf::SharedRb<ringbuf::storage::Heap<f32>>>, false, true>;

pub struct AudioBuffer {
    pub producer: ProducerCache,
    pub consumer: Option<ConsumerCache>,
}

impl AudioBuffer {
    /// Creates an audio buffer with a fixed number of samples capacity.
    pub fn new_with_capacity(capacity_samples: usize) -> Self {
        let (producer, consumer) = ringbuf::HeapRb::<f32>::new(capacity_samples).split();
        Self {
            producer,
            consumer: Some(consumer),
        }
    }

    /// Creates an audio buffer sized for `seconds` of audio at `sample_rate_hz`.
    pub fn new_with_seconds_at_rate(sample_rate_hz: u32, seconds: u32) -> Self {
        let seconds = seconds.max(1);
        let capacity = sample_rate_hz.saturating_mul(seconds) as usize;
        Self::new_with_capacity(capacity)
    }

    /// Legacy constructor with a default capacity.
    pub fn new() -> Self {
        // Default to ~1 second at 48kHz for legacy callers.
        Self::new_with_capacity(48_000)
    }

    pub fn push(&mut self, sample: f32) {
        let _result = self.producer.try_push(sample);
        #[cfg(feature = "trace")]
        if let Err(e) = _result {
            tracing::error!("Audio buffer overflow: {}", e);
        }
    }

    pub fn take_consumer(&mut self) -> Option<ConsumerCache> {
        self.consumer.take()
    }
}

impl std::fmt::Debug for AudioBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioBuffer")
            .field("has_consumer", &self.consumer.is_some())
            .finish()
    }
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new()
    }
}
