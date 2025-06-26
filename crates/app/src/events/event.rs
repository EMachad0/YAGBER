use crate::downcast::Downcastable;

pub trait Event: Downcastable + 'static {}

pub type BoxedEvent = Box<dyn Event>;
