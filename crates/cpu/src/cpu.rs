pub use crate::interface::Registers;
use crate::opcodes::Arithmetic;
use crate::opcodes::Jump;
use crate::opcodes::Load;
use crate::opcodes::Load16b;
use crate::opcodes::Logic;
use crate::opcodes::Rotate;
use crate::opcodes::Shift;
use shared::Error;

use crate::registers::futures::NextPc;
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
            registers: Registers::default(),
        }
    }

    pub fn get_memory(&self) -> Memory {
        self.memory.clone()
    }

    pub fn get_registers(&self) -> Registers {
        self.registers.clone()
    }

    async fn prefix_cb(self) -> Result<u8, Error> {
        let (opcode, cycles) = self.registers.clone().next_pc(self.memory.clone()).await?;

        if let Ok(operation) = Rotate::try_from_primitive(opcode) {
            Ok(operation.exec(self.registers, self.memory).await? + cycles)
        } else if let Ok(operation) = Shift::try_from_primitive(opcode) {
            Ok(operation.exec(self.registers, self.memory).await? + cycles)
        } else {
            Err(Error::Unimplemented)
        }
    }

    /// 1 - Get OpCode from PC
    /// 2 - Convert Opcode With Tryfrom
    /// 3 - Tryfrom to Instruction
    /// 4 - Exec Instructions -> Do the Maths put in Dest and set Flags
    pub async fn run(self) -> Result<u8, Error> {
        println!("Next Cpu Execution, fetching Opcode...!");
        let (opcode, cycles) = self.registers.clone().next_pc(self.memory.clone()).await?;

        if opcode == 0xCB {
            Ok(self.prefix_cb().await? + cycles)
        } else if let Ok(operation) = Load::try_from_primitive(opcode) {
            println!("Load 8 bits: {:#?}", operation);
            Ok(operation.exec(self.registers, self.memory).await? + cycles)
        } else if let Ok(operation) = Load16b::try_from_primitive(opcode.into()) {
            println!("Load 16b: {:#?}", operation);
            Ok(operation.exec(self.registers, self.memory).await? + cycles)
        } else if let Ok(operation) = Jump::try_from_primitive(opcode) {
            println!("Jump: {:#?}", operation);
            Ok(operation.exec(self.registers, self.memory).await? + cycles)
        } else if let Ok(operation) = Arithmetic::try_from_primitive(opcode) {
            println!("Arithmetic: {:#?}", operation);
            Ok(operation.exec(self.registers, self.memory).await? + cycles)
        } else if let Ok(operation) = Logic::try_from_primitive(opcode) {
            println!("Logic: {:#?}", operation);
            Ok(operation.exec(self.registers, self.memory).await? + cycles)
        } else {
            Err(Error::Unimplemented)
        }
    }
}
