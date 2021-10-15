pub use crate::interface::{NewRegisters, Registers};
use crate::opcodes::Arithmetic;
use crate::opcodes::Load;
use crate::opcodes::Load16b;
use crate::opcodes::Rotate;

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

    async fn prefix_cb(self) {
        let opcode: u8 = self
            .registers
            .clone()
            .next_pc(self.memory.clone())
            .await
            .unwrap();

        if let Ok(operation) = Rotate::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        }
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

        if opcode == 0xCB {
            self.prefix_cb().await;
        } else if let Ok(operation) = Load::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = Load16b::try_from_primitive(opcode.into()) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = Arithmetic::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else {
            println!("Not implemented!");
        }
        8
    }
}
