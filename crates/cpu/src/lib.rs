mod cpu;
mod instruction;
mod registers;

#[macro_use]
extern crate tracing;

pub use cpu::Cpu;
