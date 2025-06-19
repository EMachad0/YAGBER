use std::{cell::RefCell, rc::Rc};

use crate::{cram::CramInner, register::Register};

#[derive(Debug, Clone)]
pub struct CramWriterRegister {
    cram: Rc<RefCell<CramInner>>,
}

impl CramWriterRegister {
    pub fn new(cram: Rc<RefCell<CramInner>>) -> Self {
        Self { cram }
    }
}

impl Register for CramWriterRegister {
    fn read(&self) -> u8 {
        self.cram.borrow().read_specification()
    }

    fn write(&mut self, value: u8) {
        self.cram.borrow_mut().write_specification(value);
    }
}

#[derive(Debug, Clone)]
pub struct CramReaderRegister {
    cram: Rc<RefCell<CramInner>>,
}

impl CramReaderRegister {
    pub fn new(cram: Rc<RefCell<CramInner>>) -> Self {
        Self { cram }
    }
}

impl Register for CramReaderRegister {
    fn read(&self) -> u8 {
        self.cram.borrow().read_data()
    }

    fn write(&mut self, value: u8) {
        self.cram.borrow_mut().write_data(value);
    }
}
