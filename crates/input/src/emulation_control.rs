use crate::InputEvent;

pub struct EmulationControl;

impl EmulationControl {
    pub(crate) fn on_mcycle(emulator: &mut yagber_app::Emulator) {
        let event_queue = emulator
            .get_component_mut::<crate::InputEventQueue>()
            .expect("EmulationControl must be initialized after InputPlugin");

        let mut latest_event = None;
        while let Some(event) = event_queue.pop_event::<Self>() {
            let event = Self::emulation_control_event_from_input(event);
            if event.is_some() {
                latest_event = event;
            }
        }

        if let Some(event) = latest_event {
            #[cfg(feature = "trace")]
            tracing::trace!("Emulation Control: {:?}", event);
            emulator.handle_control_event(event);
        }
    }

    fn emulation_control_event_from_input(
        input: InputEvent,
    ) -> Option<yagber_app::EmulationControlEvent> {
        match input {
            InputEvent::Keyboard(keyboard_input) => {
                if keyboard_input.state == crate::KeyState::Pressed
                    && keyboard_input.key_code == crate::physical_input::keyboard::KeyCode::KeyP
                {
                    Some(yagber_app::EmulationControlEvent::TogglePause)
                } else {
                    None
                }
            }
        }
    }
}
