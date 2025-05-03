/// The result of an ALU operation.
/// Contains the result of the operation and flags indicating
/// if there was a carry from the 3rd and 7th bits.
/// Or, for subtraction, if there was a borrow from the 4th and 8th bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alu8Result {
    /// The result of the operation.
    /// Accessed by derefing the struct.
    result: u8,
    /// Carry from the 3rd bit (lower nibble).
    /// Borrow from the 4th bit (lower nibble).
    pub cb3: bool,
    /// Carry from the 7th bit (upper nibble).
    /// Borrow from the 8th bit (upper nibble).
    pub cb7: bool,
}

impl Alu8Result {
    pub fn new(result: u8, cb3: bool, cb7: bool) -> Self {
        Self { result, cb3, cb7 }
    }
}

impl std::ops::Deref for Alu8Result {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

/// Works as a service to the CPU.
/// Performs mathematical operations.
pub struct Alu8;

impl Alu8 {
    /// Adds two 8-bit numbers and returns the result.
    pub fn add(a: u8, b: u8) -> Alu8Result {
        let sum = a.wrapping_add(b);
        let carry_3 = (a & 0x0F) + (b & 0x0F) > 0x0F;
        let carry_7 = a.checked_add(b).is_none();
        Alu8Result::new(sum, carry_3, carry_7)
    }

    /// Adds two 8-bit numbers with carry and returns the result.
    pub fn adc(a: u8, b: u8, carry: u8) -> Alu8Result {
        let sum = a.wrapping_add(b).wrapping_add(carry);
        let carry_3 = (a & 0x0F) + (b & 0x0F) + carry > 0x0F;
        let carry_7 = a
            .checked_add(b)
            .and_then(|s| s.checked_add(carry))
            .is_none();
        Alu8Result::new(sum, carry_3, carry_7)
    }

    /// Subtracts two 8-bit numbers and returns the result.
    /// a - b
    pub fn sub(a: u8, b: u8) -> Alu8Result {
        let diff = a.wrapping_sub(b);
        let borrow_3 = (a & 0x0F) < (b & 0x0F);
        let borrow_7 = a < b;
        Alu8Result::new(diff, borrow_3, borrow_7)
    }

    /// Subtracts two 8-bit numbers with borrow and returns the result.
    /// a - (b + borrow)
    pub fn sbc(a: u8, b: u8, borrow: u8) -> Alu8Result {
        let res = a.wrapping_sub(b).wrapping_sub(borrow);
        let borrow_3 = (a & 0x0F) < (b & 0x0F) + borrow;
        let borrow_7 = a < b + borrow;
        Alu8Result::new(res, borrow_3, borrow_7)
    }

    /// Increments an 8-bit number and returns the result.
    pub fn inc(value: u8) -> Alu8Result {
        Self::add(value, 1)
    }

    /// Decrements an 8-bit number and returns the result.
    pub fn dec(value: u8) -> Alu8Result {
        Self::sub(value, 1)
    }

    /// Rotates an 8-bit number left and returns the result.
    /// The most significant bit is rotated into the least significant bit.
    /// The carry flag is set to the value of the most significant bit.
    pub fn rlc(value: u8) -> Alu8Result {
        let carry = value & 0x80;
        let result = value.rotate_left(1);
        Alu8Result::new(result, false, carry != 0)
    }

    /// Rotates an 8-bit number right and returns the result.
    /// The least significant bit is rotated into the most significant bit.
    /// The carry flag is set to the value of the least significant bit.
    pub fn rrc(value: u8) -> Alu8Result {
        let carry = value & 0x01;
        let result = value.rotate_right(1);
        Alu8Result::new(result, false, carry != 0)
    }

    /// Rotates an 8-bit number left through carry and returns the result.
    /// The most significant bit is rotated into the carry flag.
    /// The carry flag is rotated into the least significant bit.
    pub fn rl(value: u8, carry: u8) -> Alu8Result {
        let new_carry = value & 0x80;
        let result = (value << 1) | (carry);
        Alu8Result::new(result, false, new_carry != 0)
    }

    /// Rotates an 8-bit number right through carry and returns the result.
    /// The least significant bit is rotated into the carry flag.
    /// The carry flag is rotated into the most significant bit.
    pub fn rr(value: u8, carry: u8) -> Alu8Result {
        let new_carry = value & 0x01;
        let result = (value >> 1) | (carry << 7);
        Alu8Result::new(result, false, new_carry != 0)
    }

    /// Shifts an 8-bit number left and returns the result.
    /// arithmetic shift left.
    pub fn sla(value: u8) -> Alu8Result {
        let result = value << 1;
        let carry = value & 0x80 != 0;
        Alu8Result::new(result, false, carry)
    }

    /// Shifts an 8-bit number right and returns the result.
    /// arithmetic shift right.
    /// The most significant bit is preserved.
    pub fn sra(value: u8) -> Alu8Result {
        let result = value >> 1 | (value & 0x80);
        let carry = value & 0x01 != 0;
        Alu8Result::new(result, false, carry)
    }

    /// Shifts an 8-bit number right and returns the result.
    /// logical shift right.
    pub fn srl(value: u8) -> Alu8Result {
        let result = value >> 1;
        let carry = value & 0x01 != 0;
        Alu8Result::new(result, false, carry)
    }
}
