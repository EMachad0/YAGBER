#[derive(Debug, Default)]
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
        Self::default()
    }
    
    pub fn a(&self) -> u8 {
        self.a
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

    pub fn f(&self) -> u8 {
        self.f
    }

    pub fn h(&self) -> u8 {
        self.h
    }
    
    pub fn l(&self) -> u8 {
        self.l
    }
}