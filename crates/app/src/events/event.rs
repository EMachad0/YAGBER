pub trait Event: std::any::Any + 'static {}

pub type BoxedEvent = Box<dyn Event>;
