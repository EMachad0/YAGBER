use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::Emulator;

use super::event::Event;

/// Type alias for a boxed event handler closure.
///
/// The handler receives a mutable reference to the running [`Emulator`]
/// and a reference to the event as a trait object (`dyn Any`). The concrete
/// event type is recovered inside the closure through a down-cast.
///
/// The boxed `Fn` is `Send + Sync` so it can be shared immutably between
/// threads if the emulator ever becomes multi-threaded.
pub type BoxedHandler = Box<dyn Fn(&mut Emulator, &dyn Any) + Send + Sync + 'static>;

pub struct EventBus {
    handlers: HashMap<TypeId, Vec<BoxedHandler>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a new event type in the bus.
    ///
    /// This must be called _once_ for every event type that is going to be
    /// emitted. Attempting to register the same event twice will `panic` so
    /// that we catch configuration errors early.
    pub fn register_event<E: Event>(&mut self) {
        let id = TypeId::of::<E>();
        if self.handlers.insert(id, Vec::new()).is_some() {
            panic!("Event already registered: {}", std::any::type_name::<E>());
        }
    }

    /// Adds a handler for a concrete event type `E`.
    ///
    /// The supplied `callback` is a _free function_ (or `fn`) taking the
    /// emulator and a reference to the concrete event. We wrap that function
    /// into a trait-object that erases the concrete type so that we can store
    /// it in the same vector whatever the event is.
    pub fn add_handler<E: Event>(&mut self, callback: fn(&mut Emulator, &E)) {
        let id = TypeId::of::<E>();
        let vec = self.handlers.get_mut(&id).unwrap_or_else(|| {
            panic!(
                "Adding handler for unregistered event: {}",
                std::any::type_name::<E>()
            )
        });

        // Wrap the typed callback into an _erased_ closure.
        let wrapper: BoxedHandler = Box::new(move |emu, any_evt| {
            // Down-cast to the concrete event type. This **must** succeed
            // thanks to the `id` we looked up from the hashmap.
            let concrete_evt = any_evt
                .downcast_ref::<E>()
                .expect("Event type mismatch while dispatching");
            callback(emu, concrete_evt);
        });

        vec.push(wrapper);
    }

    /// Dispatch a single event to all of its registered handlers.
    pub fn dispatch(&self, emulator: &mut Emulator, event: &dyn Any) {
        if let Some(vec) = self.handlers.get(&event.type_id()) {
            // iterate over a *copy* of the slice to avoid borrow issues if a
            // handler decides to register more handlers while we are
            // iterating.
            for handler in vec {
                handler(emulator, event);
            }
        } else {
            panic!("Dispatching non-registered event: {:?}", event.type_id());
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus").finish()
    }
}
