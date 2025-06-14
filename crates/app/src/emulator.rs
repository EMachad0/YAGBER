use crate::{
    Component, DotCycleEvent, Event, EventBus, MCycleEvent, Plugin, TCycleEvent,
    components::ComponentBus, runners::Runner,
};

pub struct Emulator {
    cycles: u64,
    components: ComponentBus,
    event_queue: Vec<Box<dyn Event>>,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            components: ComponentBus::new(),
            event_queue: Vec::new(),
        }
        .with_default_components()
    }

    fn with_default_components(mut self) -> Self {
        // event bus
        let mut event_bus = EventBus::new();
        event_bus.register_event::<MCycleEvent>();
        event_bus.register_event::<TCycleEvent>();
        event_bus.register_event::<DotCycleEvent>();

        self.components.add_component(event_bus);
        self
    }

    /// Step the emulator by a T-Cycle or Dot cycle.
    pub fn step(&mut self) {
        self.cycles += 1;

        self.emit_event(TCycleEvent { cycle: self.cycles });
        self.emit_event(DotCycleEvent { cycle: self.cycles });
        if self.is_m_cycle() {
            let cycle = self.cycles / 4;
            self.emit_event(MCycleEvent { cycle });
        }

        while let Some(event) = self.event_queue.pop() {
            self.dispatch_event(&*event);
        }
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
            (*bus_ptr).dispatch(self, event);
        }
    }

    pub fn run<T: Runner>(self) -> T::Result {
        let runner = T::new(self);
        runner.run()
    }

    pub fn emit_event<E: Event>(&mut self, event: E) {
        self.event_queue.push(Box::new(event));
    }

    pub fn with_plugin<P: Plugin>(mut self, plugin: P) -> Self {
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

    pub fn with_event_handler<E: Event>(&mut self, handler: fn(&mut Emulator, &E)) -> &mut Self {
        let event_bus = self.components.get_component_mut::<EventBus>().unwrap();
        event_bus.add_handler::<E>(handler);
        self
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
