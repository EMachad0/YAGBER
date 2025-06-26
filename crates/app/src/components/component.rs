use crate::downcast::Downcastable;

pub trait Component: Downcastable + 'static {}
