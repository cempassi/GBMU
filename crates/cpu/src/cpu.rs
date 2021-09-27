use crate::{NewRegisters, Registers};
use memory::Memory;

#[derive(Default)]
#[allow(dead_code)]
pub struct Cpu<'a> {
    memory: Memory<'a>,
    registers: Registers,
}

impl<'a> Cpu<'a> {
    pub fn new(memory: Memory<'a>) -> Self {
        Self {
            memory,
            registers: <Registers as NewRegisters>::new(),
        }
    }

    pub fn get_registers(&self) -> Registers {
        self.registers.clone()
    }
}
