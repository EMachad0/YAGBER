use crate::downcast::Downcastable;

pub trait Event: Downcastable + 'static {}

pub type BoxedEvent = Box<dyn Event>;

impl std::fmt::Debug for dyn Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<Self>())
    }
}
