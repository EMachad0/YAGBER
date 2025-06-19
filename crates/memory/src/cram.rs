use std::{cell::RefCell, rc::Rc};

use crate::io_registers::{CramReaderRegister, CramWriterRegister};

#[derive(Default, Debug, Clone, Copy)]
struct CramSpecification {
    value: u8,
}

impl CramSpecification {
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn index(&self) -> usize {
        (self.value & 0x1F) as usize
    }

    pub fn auto_increment(&self) -> bool {
        self.value & 0x80 != 0
    }

    pub fn increment(&mut self) {
        self.value = self.value.wrapping_add(1);
    }
}

#[derive(Debug, Clone)]
pub struct CramInner {
    specification: CramSpecification,
    data: [u8; Self::SIZE],
    accessible: bool,
}

impl CramInner {
    const SIZE: usize = 64;

    pub fn new() -> Self {
        Self {
            specification: CramSpecification::default(),
            data: [0; Self::SIZE],
            accessible: true,
        }
    }

    pub fn read_specification(&self) -> u8 {
        self.specification.value()
    }

    pub fn write_specification(&mut self, value: u8) {
        self.specification.set_value(value);
    }

    pub fn read_data(&self) -> u8 {
        self.data[self.specification.index()]
    }

    pub fn write_data(&mut self, value: u8) {
        self.data[self.specification.index()] = value;
        if self.specification.auto_increment() {
            self.specification.increment();
        }
    }
}

impl Default for CramInner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Cram {
    inner: Rc<RefCell<CramInner>>,
}

impl Cram {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(CramInner::new())),
        }
    }

    pub fn writer(&self) -> CramWriterRegister {
        CramWriterRegister::new(self.inner.clone())
    }

    pub fn reader(&self) -> CramReaderRegister {
        CramReaderRegister::new(self.inner.clone())
    }

    pub fn set_accessible(&mut self, accessible: bool) {
        self.inner.borrow_mut().accessible = accessible;
    }
}

impl Default for Cram {
    fn default() -> Self {
        Self::new()
    }
}
