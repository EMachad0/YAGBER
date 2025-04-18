#[derive(Debug, Default, Clone, Copy)]
pub struct Ime {
    pub ime: bool,
}

impl Ime {
    pub fn new() -> Self {
        Self { ime: false }
    }

    pub fn set_ime(&mut self) {
        self.ime = true;
    }

    pub fn reset_ime(&mut self) {
        self.ime = false;
    }
}
