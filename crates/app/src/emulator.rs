use crate::{
    Component, Plugin, callback_queue::CallbackQueue, components::ComponentBus, runners::Runner,
};

pub struct Emulator {
    cycles: u64,
    components: ComponentBus,
    tcycle_queue: CallbackQueue,
    mcycle_queue: CallbackQueue,
    dot_cycle_queue: CallbackQueue,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            components: ComponentBus::new(),
            tcycle_queue: CallbackQueue::new(),
            mcycle_queue: CallbackQueue::new(),
            dot_cycle_queue: CallbackQueue::new(),
        }
        .with_default_components()
    }

    fn with_default_components(self) -> Self {
        self
    }

    /// Step the emulator a single frame.
    pub fn step(&mut self) {
        // A frame is 70224 dot cycles.
        for _ in 0..70224 {
            #[cfg(feature = "trace-span")]
            let _step_span = tracing::info_span!("step").entered();
            self.cycles += 1;

            self.step_dot_cycle();
            self.step_tcycle();
            if self.is_m_cycle() {
                self.step_mcycle();
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
        #[cfg(feature = "trace-span")]
        let callback_name = std::any::type_name::<F>();
        #[cfg(feature = "trace-span")]
        let callback = move |emulator: &mut Emulator| {
            let _span = tracing::info_span!("tcycle", %callback_name).entered();
            callback(emulator)
        };
        self.tcycle_queue.add_callback(callback);
        self
    }

    pub fn on_mcycle<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&mut Emulator) + Send + Sync + 'static,
    {
        #[cfg(feature = "trace-span")]
        let callback_name = std::any::type_name::<F>();
        #[cfg(feature = "trace-span")]
        let callback = move |emulator: &mut Emulator| {
            let _span = tracing::info_span!("mcycle", %callback_name).entered();
            callback(emulator)
        };
        self.mcycle_queue.add_callback(callback);
        self
    }

    pub fn on_dot_cycle<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&mut Emulator) + Send + Sync + 'static,
    {
        #[cfg(feature = "trace-span")]
        let callback_name = std::any::type_name::<F>();
        #[cfg(feature = "trace-span")]
        let callback = move |emulator: &mut Emulator| {
            let _span = tracing::info_span!("dot_cycle", %callback_name).entered();
            callback(emulator)
        };
        self.dot_cycle_queue.add_callback(callback);
        self
    }

    pub fn run<T: Runner>(self) -> T::Result {
        let runner = T::new(self);
        runner.run()
    }

    pub fn with_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        #[cfg(feature = "trace-span")]
        let _span = {
            let type_name = std::any::type_name::<P>();
            tracing::info_span!("plugin init", %type_name).entered()
        };
        plugin.init(&mut self);
        self
    }

    pub fn with_component<C: Component>(&mut self, component: C) -> &mut Self {
        self.components.add_component(component);
        self
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

    pub fn attach_component<C, F, A, R>(&mut self, f: F) -> impl Fn(A) -> R + use<C, F, A, R>
    where
        C: Component,
        F: Fn(&mut C, A) -> R,
    {
        self.components.attach_component(f)
    }

    pub fn attach_components2<C0, C1, F, A, R>(
        &mut self,
        f: F,
    ) -> impl Fn(A) -> R + use<C0, C1, F, A, R>
    where
        C0: Component,
        C1: Component,
        F: Fn(&mut C0, &mut C1, A) -> R,
    {
        self.components.attach_components2(f)
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
