pub use crate::interface::{NewRegisters, Registers};
use crate::opcodes::LoadR1R2;
use crate::opcodes::LoadRegHL;
use crate::opcodes::LoadRegNum8bit;
use crate::pc::NextPc;
use memory::Memory;
use num_enum::TryFromPrimitive;

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

    /// 1 - Get OpCode from PC
    /// 2 - Convert Opcode With Tryfrom
    /// 3 - Tryfrom to Instruction
    /// 4 - Exec Instructions -> Do the Maths put in Dest and set Flags
    pub fn run(&self) {
        let opcode = self
            .registers
            .borrow_mut()
            .pc
            .next(self.memory.clone())
            .unwrap();

        if let Ok(ope) = LoadR1R2::try_from_primitive(opcode) {
            ope.exec(self.registers.clone());
        } else if let Ok(ope) = LoadRegNum8bit::try_from_primitive(opcode) {
            ope.exec(self.registers.clone(), self.memory.clone());
        } else if let Ok(ope) = LoadRegHL::try_from_primitive(opcode) {
            ope.exec(self.registers.clone(), self.memory.clone());
        };
    }
}
