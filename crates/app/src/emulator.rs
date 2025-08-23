use crate::{
    Component, Plugin, SpeedMode, callback_queue::CallbackQueue, components::ComponentBus,
    runners::Runner,
};

pub struct Emulator {
    /// Total number of cycles since the start of emulation.
    /// emulation cycles continue to increment even when the emulator is paused.
    emulation_cycles: u64,
    /// Total number of dot cycles since the start of emulation.
    dot_cycles: u64,
    /// State
    components: ComponentBus,
    /// Callback that are called every t cycle.
    tcycle_queue: CallbackQueue,
    /// Callback that are called every m cycle.
    /// One m cycle is 4 t cycles.
    mcycle_queue: CallbackQueue,
    /// Callback that are called every dot cycle.
    /// dot cycles are not influenced by speed mode.
    dot_cycle_queue: CallbackQueue,
    /// Callbacks that are called every 1/60 second.
    /// tied to emulation cycles thus not influenced by pauses.
    fixed_step_queue: CallbackQueue,
    /// CGB speed mode.
    speed_mode: SpeedMode,
    /// Whether the emulator is paused.
    paused: bool,
}

impl Emulator {
    pub const TARGET_DOT_FREQ_HZ: u32 = 4_194_304;
    pub const NANOS_PER_DOT: u64 = 1_000_000_000 / Emulator::TARGET_DOT_FREQ_HZ as u64;

    pub fn new() -> Self {
        Self {
            emulation_cycles: 0,
            dot_cycles: 0,
            components: ComponentBus::default(),
            tcycle_queue: CallbackQueue::default(),
            mcycle_queue: CallbackQueue::default(),
            dot_cycle_queue: CallbackQueue::default(),
            fixed_step_queue: CallbackQueue::default(),
            speed_mode: SpeedMode::default(),
            paused: false,
        }
    }

    /// Step the emulator a single dot.
    pub fn step(&mut self) {
        self.emulation_cycles = self.emulation_cycles.wrapping_add(1);
        if self.emulation_cycles % (Emulator::TARGET_DOT_FREQ_HZ as u64 / 60) == 0 {
            self.step_fixed_step();
        }

        if self.paused {
            return;
        }

        #[cfg(feature = "trace-span")]
        let _step_span = tracing::info_span!("step").entered();
        self.dot_cycles = self.dot_cycles.wrapping_add(1);

        self.step_dot_cycle();
        self.step_tcycle();
        if self.is_m_cycle() {
            self.step_mcycle();
            if self.speed_mode == SpeedMode::Double {
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

    fn step_fixed_step(&mut self) {
        let emulator_ptr = self as *mut Emulator;
        let callbacks = self.fixed_step_queue.callbacks();
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

    pub fn on_fixed_step<F>(&mut self, callback: F) -> &mut Self
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
        self.fixed_step_queue.add_callback(callback);
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

    pub fn attach_components3<C0, C1, C2, F, A, R>(
        &mut self,
        f: F,
    ) -> impl Fn(A) -> R + use<C0, C1, C2, F, A, R>
    where
        C0: Component,
        C1: Component,
        C2: Component,
        F: Fn(&mut C0, &mut C1, &mut C2, A) -> R,
    {
        self.components.attach_components3(f)
    }

    pub fn get_cycles(&self) -> u64 {
        self.dot_cycles
    }

    fn is_m_cycle(&self) -> bool {
        self.dot_cycles % 4 == 0
    }

    pub fn set_speed_mode(&mut self, speed_mode: SpeedMode) {
        self.speed_mode = speed_mode;
    }

    pub fn handle_control_event(&mut self, event: crate::EmulationControlEvent) {
        #[cfg(feature = "trace")]
        tracing::trace!("Emulation Control Event: {:?}", event);
        match event {
            crate::EmulationControlEvent::Pause => self.paused = true,
            crate::EmulationControlEvent::Resume => self.paused = false,
            crate::EmulationControlEvent::TogglePause => self.paused = !self.paused,
        }
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
            .field("cycles", &self.dot_cycles)
            .finish()
    }
}
