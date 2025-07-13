use std::collections::HashMap;

use crate::components::Component;

pub struct ComponentBus {
    components: HashMap<std::any::TypeId, Box<dyn Component>>,
}

impl ComponentBus {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn add_component<T: Component>(&mut self, component: T) {
        let type_id = std::any::TypeId::of::<T>();
        self.components.insert(type_id, Box::new(component));
    }

    pub fn has_component<T: Component>(&self) -> bool {
        let type_id = std::any::TypeId::of::<T>();
        self.components.contains_key(&type_id)
    }

    pub fn get_component<T: Component>(&self) -> Option<&T> {
        let type_id = std::any::TypeId::of::<T>();
        self.components
            .get(&type_id)
            .and_then(|c| c.as_ref().as_any_ref().downcast_ref::<T>())
    }

    pub fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        let type_id = std::any::TypeId::of::<T>();
        self.components
            .get_mut(&type_id)
            .and_then(|c| c.as_mut().as_any_mut().downcast_mut::<T>())
    }

    pub fn get_components_mut2<C0: Component, C1: Component>(
        &mut self,
    ) -> Option<(&mut C0, &mut C1)> {
        use std::any::TypeId;
        let keys = [&TypeId::of::<C0>(), &TypeId::of::<C1>()];
        if keys[0] == keys[1] {
            return None;
        }

        let [opt0, opt1] = self.components.get_disjoint_mut(keys);
        match (opt0, opt1) {
            (Some(c0_box), Some(c1_box)) => {
                let c0_ref = Self::downcast_box_mut::<C0>(c0_box);
                let c1_ref = Self::downcast_box_mut::<C1>(c1_box);
                Some((c0_ref, c1_ref))
            }
            _ => None,
        }
    }

    pub fn get_components_mut3<C0: Component, C1: Component, C2: Component>(
        &mut self,
    ) -> Option<(&mut C0, &mut C1, &mut C2)> {
        use std::any::TypeId;
        let keys = [
            &TypeId::of::<C0>(),
            &TypeId::of::<C1>(),
            &TypeId::of::<C2>(),
        ];
        if keys[0] == keys[1] || keys[0] == keys[2] || keys[1] == keys[2] {
            return None;
        }

        let [opt0, opt1, opt2] = self.components.get_disjoint_mut(keys);
        match (opt0, opt1, opt2) {
            (Some(c0_box), Some(c1_box), Some(c2_box)) => {
                let c0_ref = Self::downcast_box_mut::<C0>(c0_box);
                let c1_ref = Self::downcast_box_mut::<C1>(c1_box);
                let c2_ref = Self::downcast_box_mut::<C2>(c2_box);
                Some((c0_ref, c1_ref, c2_ref))
            }
            _ => None,
        }
    }

    fn downcast_box_mut<T: Component>(boxed_component: &mut Box<dyn Component>) -> &mut T {
        boxed_component
            .as_mut()
            .as_any_mut()
            .downcast_mut::<T>()
            .expect("Failed to downcast component")
    }

    pub fn attach_component<C, F, A, R>(&mut self, f: F) -> impl Fn(A) -> R + use<C, F, A, R>
    where
        C: Component,
        F: Fn(&mut C, A) -> R,
    {
        let component = self.get_component_mut::<C>().expect("Component not found");
        // SAFETY: Component stays alive inside the emulator
        let component = component as *mut C;
        move |a| f(unsafe { &mut *component }, a)
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
        let (component0, component1) = self
            .get_components_mut2::<C0, C1>()
            .expect("Components not found");
        // SAFETY: Component stays alive inside the emulator
        let component0 = component0 as *mut C0;
        let component1 = component1 as *mut C1;
        move |a| f(unsafe { &mut *component0 }, unsafe { &mut *component1 }, a)
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
        let (component0, component1, component2) = self
            .get_components_mut3::<C0, C1, C2>()
            .expect("Components not found");
        // SAFETY: Component stays alive inside the emulator
        let component0 = component0 as *mut C0;
        let component1 = component1 as *mut C1;
        let component2 = component2 as *mut C2;
        move |a| {
            f(
                unsafe { &mut *component0 },
                unsafe { &mut *component1 },
                unsafe { &mut *component2 },
                a,
            )
        }
    }
}

impl Default for ComponentBus {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ComponentBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentBus").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent {
        pub value: u8,
    }

    struct TestComponent2 {
        pub value: u8,
    }

    impl Component for TestComponent {}
    impl Component for TestComponent2 {}

    #[test]
    fn test_get_component() {
        let mut bus = ComponentBus::new();
        bus.add_component(TestComponent { value: 42 });
        bus.add_component(TestComponent2 { value: 43 });
        let component = bus
            .get_component::<TestComponent>()
            .expect("Component not found");
        assert_eq!(component.value, 42);
    }

    #[test]
    fn test_get_component_mut() {
        let mut bus = ComponentBus::new();

        bus.add_component(TestComponent { value: 42 });

        let component = bus
            .get_component_mut::<TestComponent>()
            .expect("Component not found");
        assert_eq!(component.value, 42);
        component.value = 43;

        let component = bus
            .get_component::<TestComponent>()
            .expect("Component not found");
        assert_eq!(component.value, 43);
    }

    #[test]
    fn test_get_components_mut2() {
        let mut bus = ComponentBus::new();
        bus.add_component(TestComponent { value: 42 });
        bus.add_component(TestComponent2 { value: 43 });
        let (component1, component2) = bus
            .get_components_mut2::<TestComponent, TestComponent2>()
            .expect("Components not found");
        assert_eq!(component1.value, 42);
        assert_eq!(component2.value, 43);
    }

    #[test]
    fn test_get_components_mut2_same_type() {
        let mut bus = ComponentBus::new();
        bus.add_component(TestComponent { value: 42 });
        let ret = bus.get_components_mut2::<TestComponent, TestComponent>();
        assert!(ret.is_none());
    }
}
