#[derive(Debug, Clone, Copy)]
pub enum EdgeMode {
    Rising,
    Falling,
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeDetector {
    value: bool,
    mode: EdgeMode,
}

impl EdgeDetector {
    pub fn new(mode: EdgeMode) -> Self {
        Self { value: false, mode }
    }

    pub fn tick(&mut self, value: bool) -> bool {
        let result = match self.mode {
            EdgeMode::Rising => !self.value && value,
            EdgeMode::Falling => self.value && !value,
        };
        self.value = value;
        result
    }

    pub fn set_value(&mut self, value: bool) {
        self.value = value;
    }
}
