use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    #[allow(dead_code)]
    pub fn f(&self) -> u8 {
        self.f
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn c(&self) -> u8 {
        self.c
    }

    pub fn d(&self) -> u8 {
        self.d
    }

    pub fn e(&self) -> u8 {
        self.e
    }

    pub fn h(&self) -> u8 {
        self.h
    }

    pub fn l(&self) -> u8 {
        self.l
    }

    pub fn flags(&self) -> FlagRegister {
        FlagRegister::new(self.f)
    }

    pub fn flags_mut(&mut self) -> FlagRegisterMut {
        FlagRegisterMut::new(&mut self.f)
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    pub fn af(&self) -> u16 {
        u16::from_le_bytes([self.f, self.a])
    }

    pub fn bc(&self) -> u16 {
        u16::from_le_bytes([self.c, self.b])
    }

    pub fn de(&self) -> u16 {
        u16::from_le_bytes([self.e, self.d])
    }

    pub fn hl(&self) -> u16 {
        u16::from_le_bytes([self.l, self.h])
    }

    pub fn hl_inc(&mut self) -> u16 {
        let hl = self.hl();
        self.set_hl(hl.wrapping_add(1));
        hl
    }

    pub fn hl_dec(&mut self) -> u16 {
        let hl = self.hl();
        self.set_hl(hl.wrapping_sub(1));
        hl
    }

    pub fn set_af(&mut self, value: u16) {
        let [f, a] = value.to_le_bytes();
        self.a = a;
        self.f = f & 0xF0;
    }

    pub fn set_bc(&mut self, value: u16) {
        let [c, b] = value.to_le_bytes();
        self.b = b;
        self.c = c;
    }

    pub fn set_de(&mut self, value: u16) {
        let [e, d] = value.to_le_bytes();
        self.d = d;
        self.e = e;
    }

    pub fn set_hl(&mut self, value: u16) {
        let [l, h] = value.to_le_bytes();
        self.h = h;
        self.l = l;
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

/// A "view" onto a single byte.
pub struct FlagRegister {
    reg: u8,
}

impl FlagRegister {
    pub fn new(reg: u8) -> Self {
        Self { reg }
    }

    pub fn z(&self) -> bool {
        self.reg & 0b1000_0000 != 0
    }

    pub fn n(&self) -> bool {
        self.reg & 0b0100_0000 != 0
    }

    pub fn h(&self) -> bool {
        self.reg & 0b0010_0000 != 0
    }

    pub fn c(&self) -> bool {
        self.c_u8() != 0
    }

    pub fn c_u8(&self) -> u8 {
        (self.reg & 0b0001_0000) >> 4
    }
}

impl Debug for FlagRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FlagRegister")
            .field("z", &self.z())
            .field("n", &self.n())
            .field("h", &self.h())
            .field("c", &self.c())
            .finish()
    }
}

pub struct FlagRegisterMut<'r> {
    reg: &'r mut u8,
}

impl<'r> FlagRegisterMut<'r> {
    pub fn new(reg: &'r mut u8) -> Self {
        Self { reg }
    }

    pub fn set_z(self, value: bool) -> Self {
        if value {
            *self.reg |= 0b1000_0000;
        } else {
            *self.reg &= !0b1000_0000;
        }
        self
    }

    pub fn set_z_if_zero(self, value: u8) -> Self {
        self.set_z(value == 0)
    }

    pub fn set_n(self, value: bool) -> Self {
        if value {
            *self.reg |= 0b0100_0000;
        } else {
            *self.reg &= !0b0100_0000;
        }
        self
    }

    pub fn set_h(self, value: bool) -> Self {
        if value {
            *self.reg |= 0b0010_0000;
        } else {
            *self.reg &= !0b0010_0000;
        }
        self
    }

    pub fn set_c(self, value: bool) -> Self {
        if value {
            *self.reg |= 0b0001_0000;
        } else {
            *self.reg &= !0b0001_0000;
        }
        self
    }
}
