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

    pub fn get_components_mut2<C1: Component, C2: Component>(
        &mut self,
    ) -> Option<(&mut C1, &mut C2)> {
        use std::any::TypeId;
        let keys = [&TypeId::of::<C1>(), &TypeId::of::<C2>()];
        if keys[0] == keys[1] {
            return None;
        }

        let [opt1, opt2] = self.components.get_disjoint_mut(keys);
        match (opt1, opt2) {
            (Some(c1_box), Some(c2_box)) => {
                let c1_ref = c1_box
                    .as_mut()
                    .as_any_mut()
                    .downcast_mut::<C1>()
                    .expect("Failed to downcast C1");
                let c2_ref = c2_box
                    .as_mut()
                    .as_any_mut()
                    .downcast_mut::<C2>()
                    .expect("Failed to downcast C2");
                Some((c1_ref, c2_ref))
            }
            _ => None,
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
