use arbitrary_int::u2;

#[derive(Debug, Default, Clone, Copy)]
pub struct ConditionCode(u2);

impl ConditionCode {
    pub fn new(value: u2) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0.value()
    }
}
