pub use crate::interface::Registers;
use crate::opcodes::decode::{Decode, Decoder};
use crate::opcodes::Arithmetic;
use crate::opcodes::Arithmetic16b;
use crate::opcodes::Control;
use crate::opcodes::Jump;
use crate::opcodes::Load;
use crate::opcodes::Load16b;
use crate::opcodes::Logic;
use shared::Error;

use crate::registers::futures::{AsyncGet, Get};
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

    async fn decode(self, opcode: u8) -> Result<Decode, Error> {
        if let Ok(operation) = Control::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory))
        } else if let Ok(operation) = Load::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory))
        } else if let Ok(operation) = Load16b::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory))
        } else if let Ok(operation) = Jump::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory))
        } else if let Ok(operation) = Arithmetic::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory))
        } else if let Ok(operation) = Arithmetic16b::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory))
        } else if let Ok(operation) = Logic::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory))
        } else {
            println!("Something went wrong?, opcode: {}", opcode);
            Err(Error::Unimplemented)
        }
    }

    pub async fn run(self) -> Result<u8, Error> {
        let (opcode, cycles) = Get::Next
            .get(self.registers.clone(), self.memory.clone())
            .await?;
        println!("New Cpu Execution, Opcode: {:#X}", opcode);

        let execute = self.decode(opcode).await?;
        Ok(execute.await? + cycles)
    }
}
