use std::any::Any;

pub trait Component: Any + 'static {
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + 'static> Component for T {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
