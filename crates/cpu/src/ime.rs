#[derive(Debug, Default, Clone, Copy)]
pub struct Ime {
    ime: bool,
    ei_delay: Option<u8>,
    interrupt_handling: bool,
}

impl Ime {
    pub fn set_ime(&mut self) {
        self.ei_delay = Some(2);
    }

    pub fn update_ime(&mut self) {
        if let Some(delay) = self.ei_delay {
            if delay == 1 {
                self.ime = true;
                self.ei_delay = None;
            } else {
                self.ei_delay = Some(delay - 1);
            }
        }
    }

    pub fn reset_ime(&mut self) {
        self.ime = false;
        self.ei_delay = None;
    }

    pub fn set_interrupt_handling(&mut self) {
        self.interrupt_handling = true;
        self.ime = false;
    }

    pub fn reset_interrupt_handling(&mut self) {
        self.interrupt_handling = false;
    }

    pub fn ime(&self) -> bool {
        self.ime
    }

    pub fn interrupt_handling(&self) -> bool {
        self.interrupt_handling
    }
}
