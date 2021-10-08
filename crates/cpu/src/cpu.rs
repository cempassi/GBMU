pub use crate::interface::{NewRegisters, Registers};
use crate::opcodes::Pop;
use crate::opcodes::Push;
use crate::opcodes::RotateLeft;

use crate::area::Bits16;
use crate::nextpc::NextPc;
use crate::RegisterBus;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::Error;

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

        if let Ok(operation) = RotateLeft::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        }
    }

    /// Pops a 16-bit value from the stack, updating the stack pointer register.
    pub async fn pop(registers: Registers, memory: Memory) -> Result<u16, Error> {
        let data = registers.borrow().get(Bits16::SP);
        registers.borrow_mut().set(Bits16::SP, data.wrapping_add(2));
        memory.borrow().get_u16(data)
    }

    /// Pushes a 16-bit value to the stack, updating the stack pointer register.
    pub async fn push(registers: Registers, memory: Memory, data: u16) -> Result<(), Error> {
        let dst = registers.borrow().get(Bits16::SP);
        registers.borrow_mut().set(Bits16::SP, dst.wrapping_sub(2));
        memory.borrow_mut().set_u16(dst, data)
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
        } else if let Ok(operation) = Pop::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = Push::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else {
            println!("Not implemented!");
        }
        8
    }
}
