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
    const CAPACITY: usize = 1 << 16;

    pub fn new() -> Self {
        let (producer, consumer) = ringbuf::HeapRb::<f32>::new(Self::CAPACITY).split();
        Self {
            producer,
            consumer: Some(consumer),
        }
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
