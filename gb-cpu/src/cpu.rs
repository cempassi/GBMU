mod area;
mod flags;
mod opcodes;
mod pc;
mod registers;

use crate::error::Error;
use gb_memory::Memory;
use opcodes::LoadRegNum8bit;
use pc::NextPc;
pub use registers::Registers;
use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn step(&mut self, memory: &mut Memory) -> Result<u32, Error> {
        let opcode = self.registers.pc.next(memory)?;

        if let Ok(load) = LoadRegNum8bit::try_from(opcode) {
            load.proceed(&mut self.registers, memory)
        } else {
            Err(Error::Unimplemented)
        }
    }
}
