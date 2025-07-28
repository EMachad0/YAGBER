use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

#[derive(Debug, Default, Clone)]
pub struct AudioBuffer {
    pub buffer: Arc<Mutex<VecDeque<f32>>>,
}

impl AudioBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn push(&self, left: f32, right: f32) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push_back(left);
        buffer.push_back(right);
    }

    pub fn lock(&self) -> std::sync::MutexGuard<VecDeque<f32>> {
        self.buffer.lock().unwrap()
    }
}
