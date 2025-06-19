use std::{cell::RefCell, rc::Rc};

use crate::events::{EventSender, event::BoxedEvent};

pub struct EventQueue {
    inner: Rc<RefCell<Vec<BoxedEvent>>>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn pop(&self) -> Option<BoxedEvent> {
        self.inner.borrow_mut().pop()
    }

    pub fn sender(&self) -> EventSender {
        EventSender::new(self.inner.clone())
    }
}
