pub use crate::interface::Registers;
use crate::opcodes::decode::{Decode, Decoder};
use crate::opcodes::Arithmetic;
use crate::opcodes::Jump;
use crate::opcodes::Load;
use crate::opcodes::Load16b;
use crate::opcodes::Logic;
use crate::opcodes::Rotate;
use crate::opcodes::Shift;
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

    fn prefix_cb(self, opcode: u8) -> Result<Decode, Error> {
        if let Ok(operation) = Rotate::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory))
        } else if let Ok(operation) = Shift::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory))
        } else {
            Err(Error::Unimplemented)
        }
    }

    async fn decode(self, opcode: u8, cycles: &mut u8) -> Result<Decode, Error> {
        if opcode == 0xCB {
            let (opcode, cb_cycles) = Get::Next
                .get(self.registers.clone(), self.memory.clone())
                .await?;

            *cycles += cb_cycles;
            self.prefix_cb(opcode)
        } else if let Ok(operation) = Load::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory.clone()))
        } else if let Ok(operation) = Load16b::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory.clone()))
        } else if let Ok(operation) = Jump::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory.clone()))
        } else if let Ok(operation) = Arithmetic::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory.clone()))
        } else if let Ok(operation) = Logic::try_from_primitive(opcode) {
            Ok(operation.decode(self.registers.clone(), self.memory.clone()))
        } else {
            Err(Error::Unimplemented)
        }
    }

    pub async fn run(self) -> Result<u8, Error> {
        println!("Next Cpu Execution, fetching Opcode...!");
        let (opcode, mut cycles) = Get::Next
            .get(self.registers.clone(), self.memory.clone())
            .await?;

        let execute = self.decode(opcode, &mut cycles).await?;
        Ok(execute.await? + cycles)
    }
}
