pub mod area;
mod flags;
mod opcodes;
mod pc;
mod registers;
use registers::New;

use memory::Memory;
pub use registers::RcRegisters as Registers;

#[derive(Default)]
pub struct Cpu<'a> {
    memory: Memory<'a>,
    registers: Registers,
}

impl<'a> Cpu<'a> {
    pub fn new(memory: Memory<'a>) -> Self {
        Self {
            memory,
            registers: <Registers as New>::new(),
        }
    }

    pub fn get_registers(&self) -> Registers {
        self.registers.clone()
    }
}
