use std::collections::VecDeque;

use crate::InputEvent;

const QUEUE_CAPACITY: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct EventQueueKey {
    value: std::any::TypeId,
}

impl EventQueueKey {
    pub fn new<T: 'static>() -> Self {
        Self {
            value: std::any::TypeId::of::<T>(),
        }
    }
}

#[derive(Debug, Clone)]
struct InnerQueue {
    events: VecDeque<InputEvent>,
}

impl InnerQueue {
    pub fn new() -> Self {
        Self {
            events: VecDeque::with_capacity(QUEUE_CAPACITY),
        }
    }

    pub fn push_event(&mut self, event: InputEvent) {
        if self.events.len() + 1 >= QUEUE_CAPACITY {
            let _dropped_event = self.events.pop_front();
            #[cfg(feature = "trace")]
            tracing::warn!("Event queue is full, dropping event: {_dropped_event:?}");
        } 
        self.events.push_back(event);
    }

    pub fn pop_event(&mut self) -> Option<InputEvent> {
        self.events.pop_front()
    }
}

impl Default for InnerQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone)]
pub struct InputEventQueue {
    queues: ahash::AHashMap<EventQueueKey, InnerQueue>,
}

impl InputEventQueue {
    pub fn add_observer<T: 'static>(&mut self) {
        let key = EventQueueKey::new::<T>();
        self.queues.entry(key).or_default();
    }

    pub fn with_observer<T: 'static>(&mut self) -> &mut Self {
        self.add_observer::<T>();
        self
    }

    pub fn push_event(&mut self, event: InputEvent) {
        for queue in self.queues.values_mut() {
            queue.push_event(event.clone());
        }
    }

    pub fn pop_event<T: 'static>(&mut self) -> Option<InputEvent> {
        let key = EventQueueKey::new::<T>();
        if let Some(queue) = self.queues.get_mut(&key) {
            queue.pop_event()
        } else {
            #[cfg(feature = "trace")]
            tracing::error!(
                "No observer found for type: {:?}",
                std::any::type_name::<T>()
            );
            None
        }
    }
}

impl yagber_app::Component for InputEventQueue {}
