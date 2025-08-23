use crate::Emulator;

pub type BoxedHandler = Box<dyn Fn(&mut Emulator) + Send + Sync + 'static>;

pub struct CallbackQueue {
    callbacks: Vec<BoxedHandler>,
}

impl CallbackQueue {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: Fn(&mut Emulator) + Send + Sync + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    pub fn callbacks(&self) -> &[BoxedHandler] {
        &self.callbacks
    }
}

impl Default for CallbackQueue {
    fn default() -> Self {
        Self::new()
    }
}
