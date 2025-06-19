use std::{cell::RefCell, rc::Rc};

use crate::{Event, events::event::BoxedEvent};

#[derive(Clone)]
pub struct EventSender {
    queue: Rc<RefCell<Vec<BoxedEvent>>>,
}

impl EventSender {
    pub fn new(queue: Rc<RefCell<Vec<BoxedEvent>>>) -> Self {
        Self { queue }
    }

    pub fn send<E: Event>(&self, event: E) {
        self.queue.borrow_mut().push(Box::new(event));
    }
}

impl std::fmt::Debug for EventSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventSender")
            .field("queue", &"...")
            .finish()
    }
}
