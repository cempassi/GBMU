pub use crate::interface::{NewRegisters, Registers};
use crate::opcodes::consts;
use crate::opcodes::AddRegA;
use crate::opcodes::Call;
use crate::opcodes::Jump;
use crate::opcodes::Pop;
use crate::opcodes::Push;
use crate::opcodes::RelJump;
use crate::opcodes::Restart;
use crate::opcodes::Return;
use crate::opcodes::RotateLeft;

use crate::area::{Bits16, Flag};
use crate::nextpc::NextPc;
use crate::RegisterBus;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;
use shared::Error;

#[derive(Default, Clone)]
pub struct Cpu {
    memory: Memory,
    registers: Registers,
    interrupts: bool,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            registers: <Registers as NewRegisters>::new(),
            interrupts: false,
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

    /// JPNZ = 0xc2     RETNZ = 0xc0,
    /// JPZ = 0xcA      RETNC = 0xd0,
    /// JPNC = 0xd2     RETZ = 0xc8,
    /// JPC = 0xda      RETC = 0xd8,
    pub fn flags_conditions(opcode: u8, registers: Registers) -> bool {
        match opcode {
            consts::JUMP_NO_Z | consts::RET_NO_Z => !registers.borrow().get(Flag::Z),
            consts::JUMP_Z | consts::RET_Z => registers.borrow().get(Flag::Z),
            consts::JUMP_NO_C | consts::RET_NO_C => !registers.borrow().get(Flag::C),
            consts::JUMP_C | consts::RET_C => registers.borrow().get(Flag::C),
            _ => false,
        }
    }

    /// Pops a 16-bit value from the stack, updating the stack pointer register.
    pub async fn pop(registers: Registers, memory: Memory) -> Result<u16, Error> {
        let dst = registers.borrow().get(Bits16::SP);
        registers.borrow_mut().set(Bits16::SP, dst.wrapping_add(2));
        Ok(<Memory as Async<u16>>::get(memory, dst).await.unwrap())
    }

    /// Pushes a 16-bit value to the stack, updating the stack pointer register.
    pub async fn push(registers: Registers, memory: Memory, data: u16) -> Result<(), Error> {
        let dst = registers.borrow().get(Bits16::SP);
        registers.borrow_mut().set(Bits16::SP, dst.wrapping_sub(2));
        <Memory as Async<u16>>::set(memory, dst, data)
            .await
            .unwrap();
        Ok(())
    }

    ///Jump to a 16 bit Address pointed by Data
    pub fn jump(registers: Registers, data: u16) {
        registers.borrow_mut().set(Bits16::PC, data);
    }

    /// 1 - Get OpCode from PC
    /// 2 - Convert Opcode With Tryfrom
    /// 3 - Tryfrom to Instruction
    /// 4 - Exec Instructions -> Do the Maths put in Dest and set Flags
    pub async fn run(mut self) -> u8 {
        let opcode: u8 = self
            .registers
            .clone()
            .next_pc(self.memory.clone())
            .await
            .unwrap();

        if opcode == 0xCB {
            self.prefix_cb().await;
        } else if let Ok(operation) = Call::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = AddRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = Pop::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = Push::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = Jump::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = RelJump::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = Restart::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = Return::try_from_primitive(opcode) {
            if opcode == 0xd9 {
                self.interrupts = true
            }
            operation.exec(self.registers, self.memory).await;
        } else {
            println!("Not implemented!");
        }
        8
    }
}
