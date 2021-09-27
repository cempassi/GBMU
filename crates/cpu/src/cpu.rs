use crate::{NewRegisters, Registers};
use memory::Memory;

#[derive(Default)]
#[allow(dead_code)]
pub struct Cpu {
    memory: Memory,
    registers: Registers,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            registers: <Registers as NewRegisters>::new(),
        }
    }

    pub fn get_registers(&self) -> Registers {
        self.registers.clone()
    }
}
