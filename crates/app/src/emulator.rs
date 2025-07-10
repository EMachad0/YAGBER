use crate::{
    Component, Event, EventBus, Plugin, callback_queue::CallbackQueue, components::ComponentBus,
    events::EventQueue, runners::Runner,
};

pub struct Emulator {
    cycles: u64,
    components: ComponentBus,
    event_queue: EventQueue,
    tcycle_queue: CallbackQueue,
    mcycle_queue: CallbackQueue,
    dot_cycle_queue: CallbackQueue,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            components: ComponentBus::new(),
            event_queue: EventQueue::new(),
            tcycle_queue: CallbackQueue::new(),
            mcycle_queue: CallbackQueue::new(),
            dot_cycle_queue: CallbackQueue::new(),
        }
        .with_default_components()
    }

    fn with_default_components(mut self) -> Self {
        self.components.add_component(EventBus::new());
        self
    }

    /// Step the emulator a single frame.
    pub fn step(&mut self) {
        // A frame is 72224 dot cycles.
        for _ in 0..72224 {
            #[cfg(feature = "trace")]
            let _step_span = tracing::info_span!("step").entered();
            self.cycles += 1;

            self.step_dot_cycle();
            self.step_tcycle();
            if self.is_m_cycle() {
                self.step_mcycle();
            }

            while let Some(event) = self.event_queue.pop() {
                self.dispatch_event(&*event);
            }
        }
    }

    fn step_tcycle(&mut self) {
        let emulator_ptr = self as *mut Emulator;

        let callbacks = self.tcycle_queue.callbacks();
        for callback in callbacks {
            callback(unsafe { &mut *emulator_ptr });
        }
    }

    fn step_mcycle(&mut self) {
        let emulator_ptr = self as *mut Emulator;

        let callbacks = self.mcycle_queue.callbacks();
        for callback in callbacks {
            callback(unsafe { &mut *emulator_ptr });
        }
    }

    fn step_dot_cycle(&mut self) {
        let emulator_ptr = self as *mut Emulator;

        let callbacks = self.dot_cycle_queue.callbacks();
        for callback in callbacks {
            callback(unsafe { &mut *emulator_ptr });
        }
    }

    pub fn on_tcycle<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&mut Emulator) + Send + Sync + 'static,
    {
        self.tcycle_queue.add_callback(callback);
        self
    }

    pub fn on_mcycle<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&mut Emulator) + Send + Sync + 'static,
    {
        self.mcycle_queue.add_callback(callback);
        self
    }

    pub fn on_dot_cycle<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&mut Emulator) + Send + Sync + 'static,
    {
        self.dot_cycle_queue.add_callback(callback);
        self
    }

    fn dispatch_event(&mut self, event: &dyn Event) {
        // We cannot keep an immutable borrow of `self` (through
        // `get_component`) while also passing `&mut self` to the
        // dispatched handlers.
        // To work around this we obtain a *raw pointer* to the `EventBus`,
        // _drop_ the immutable borrow immediately (by ending the scope),
        // and then invoke `dispatch` through the raw pointer.

        let bus_ptr = {
            // Immutable borrow is *temporary* and ends right after this
            // block, so it won't overlap with the upcoming mutable
            // borrow of `self`.
            let bus_ref = self
                .components
                .get_component::<EventBus>()
                .expect("EventBus not found");
            bus_ref as *const EventBus
        };

        // SAFETY: `bus_ptr` is valid for the entire lifetime of the
        // emulator because components never move in memory after being
        // inserted into the `ComponentBus`.
        unsafe {
            (*bus_ptr).dispatch(self, event.as_any_ref());
        }
    }

    pub fn run<T: Runner>(self) -> T::Result {
        let runner = T::new(self);
        runner.run()
    }

    pub fn with_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        #[cfg(feature = "trace")]
        {
            let type_name = std::any::type_name::<P>();
            let _span = tracing::info_span!("plugin init", %type_name).entered();
        }
        plugin.init(&mut self);
        self
    }

    pub fn with_component<C: Component>(&mut self, component: C) -> &mut Self {
        self.components.add_component(component);
        self
    }

    pub fn with_event<E: Event>(&mut self) -> &mut Self {
        let event_bus = self.components.get_component_mut::<EventBus>().unwrap();
        event_bus.register_event::<E>();
        self
    }

    pub fn with_event_handler<E, F>(&mut self, handler: F) -> &mut Self
    where
        E: Event,
        F: Fn(&mut Emulator, &E) + Send + Sync + 'static,
    {
        let event_bus = self.components.get_component_mut::<EventBus>().unwrap();
        event_bus.add_handler(handler);
        self
    }

    pub fn event_sender(&self) -> crate::events::EventSender {
        self.event_queue.sender()
    }

    pub fn has_component<C: Component>(&self) -> bool {
        self.components.has_component::<C>()
    }

    pub fn get_component_mut<C: Component>(&mut self) -> Option<&mut C> {
        self.components.get_component_mut::<C>()
    }

    pub fn get_components_mut2<C1: Component, C2: Component>(
        &mut self,
    ) -> Option<(&mut C1, &mut C2)> {
        self.components.get_components_mut2::<C1, C2>()
    }

    pub fn get_component<C: Component>(&self) -> Option<&C> {
        self.components.get_component::<C>()
    }

    pub fn get_cycles(&self) -> u64 {
        self.cycles
    }

    fn is_m_cycle(&self) -> bool {
        self.cycles % 4 == 0
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Emulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emulator")
            .field("cycles", &self.cycles)
            .finish()
    }
}
