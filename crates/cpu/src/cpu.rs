pub use crate::interface::{NewRegisters, Registers};
use crate::opcodes::LoadHL8b;
use crate::opcodes::LoadR1R2;
use crate::opcodes::LoadR8b;
use crate::opcodes::LoadRR16b;
use crate::opcodes::LoadRegHL;

use crate::nextpc::NextPc;
use memory::Memory;
use num_enum::TryFromPrimitive;

#[derive(Default, Clone)]
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

    pub fn get_memory(&self) -> Memory {
        self.memory.clone()
    }

    pub fn get_registers(&self) -> Registers {
        self.registers.clone()
    }

    /// 1 - Get OpCode from PC
    /// 2 - Convert Opcode With Tryfrom
    /// 3 - Tryfrom to Instruction
    /// 4 - Exec Instructions -> Do the Maths put in Dest and set Flags
    pub async fn run(self) -> u8 {
        let opcode: u8 = self
            .registers
            .clone()
            .next_pc(self.memory.clone())
            .await
            .unwrap();

        if let Ok(operation) = LoadR1R2::try_from_primitive(opcode) {
            operation.exec(self.registers.clone());
        } else if let Ok(operation) = LoadR8b::try_from_primitive(opcode) {
            operation
                .exec(self.registers.clone(), self.memory.clone())
                .await;
        } else if let Ok(operation) = LoadRR16b::try_from_primitive(opcode.into()) {
            operation
                .exec(self.registers.clone(), self.memory.clone())
                .await;
        } else if let Ok(operation) = LoadHL8b::try_from_primitive(opcode) {
            operation
                .exec(self.registers.clone(), self.memory.clone())
                .await;
        } else if let Ok(operation) = LoadRegHL::try_from_primitive(opcode) {
            operation
                .exec(self.registers.clone(), self.memory.clone())
                .await;
        } else {
            println!("Not implemented!");
        }
        println!("{:#?}", self.registers.borrow());
        8
    }
}
