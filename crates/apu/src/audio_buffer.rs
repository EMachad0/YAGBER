use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct AudioBuffer {
    pub buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, sample: f32) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(sample);
    }

    pub fn lock(&self) -> std::sync::MutexGuard<Vec<f32>> {
        self.buffer.lock().unwrap()
    }

    pub fn drain(&self) -> Vec<f32> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.drain(..).collect()
    }
}
