/// The result of an ALU operation.
/// Contains the result of the operation and flags indicating
/// if there was a carry from the 11th and 15th bits.
pub struct Alu16Result {
    /// The result of the operation.
    /// Accessed by derefing the struct.
    result: u16,
    /// Carry from the 11th bit (lower nibble).
    /// Borrow from the 12th bit (lower nibble).
    pub cb11: bool,
    /// Carry from the 15th bit (upper nibble).
    /// Borrow from the 16th bit (upper nibble).
    pub cb15: bool,
}

impl Alu16Result {
    pub fn new(result: u16, cb11: bool, cb15: bool) -> Self {
        Self {
            result,
            cb11,
            cb15,
        }
    }
}

impl std::ops::Deref for Alu16Result {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

pub struct Alu16;

impl Alu16 {
    /// Adds two 16-bit numbers and returns the result.
    pub fn add(a: u16, b: u16) -> Alu16Result {
        let sum = a.wrapping_add(b);
        let carry_11 = (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF;
        let carry_15 = a.checked_add(b).is_none();
        Alu16Result::new(sum, carry_11, carry_15)
    }

    /// Adds two 16-bit numbers with carry and returns the result.
    pub fn adc(a: u16, b: u16, carry: u8) -> Alu16Result {
        let sum = a.wrapping_add(b).wrapping_add(carry as u16);
        let carry_11 = (a & 0x0FFF) + (b & 0x0FFF) + (carry as u16) > 0x0FFF;
        let carry_15 = a
            .checked_add(b)
            .and_then(|s| s.checked_add(carry as u16))
            .is_none();
        Alu16Result::new(sum, carry_11, carry_15)
    }
}