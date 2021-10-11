pub use crate::interface::{NewRegisters, Registers};
use crate::opcodes::consts;
use crate::opcodes::AddRegA;
use crate::opcodes::AddRegHL;
use crate::opcodes::AddRegSP;
use crate::opcodes::AndRegA;
use crate::opcodes::CPLRegA;
use crate::opcodes::Call;
use crate::opcodes::CmpRegA;
use crate::opcodes::DecAdjustRegA;
use crate::opcodes::DecRegN;
use crate::opcodes::DecRegNN;
use crate::opcodes::IncRegN;
use crate::opcodes::IncRegNN;
use crate::opcodes::Jump;
use crate::opcodes::LoadBCDERegA;
use crate::opcodes::LoadHL8b;
use crate::opcodes::LoadHLMRegA;
use crate::opcodes::LoadHLPRegA;
use crate::opcodes::LoadHLReg;
use crate::opcodes::LoadMem16bRegA;
use crate::opcodes::LoadMem16bRegSP;
use crate::opcodes::LoadMem8bRegA;
use crate::opcodes::LoadMemCRegA;
use crate::opcodes::LoadR1R2;
use crate::opcodes::LoadR8b;
use crate::opcodes::LoadRR16b;
use crate::opcodes::LoadRegABCDE;
use crate::opcodes::LoadRegAHLM;
use crate::opcodes::LoadRegAHLP;
use crate::opcodes::LoadRegAMem16b;
use crate::opcodes::LoadRegAMem8b;
use crate::opcodes::LoadRegAMemC;
use crate::opcodes::LoadRegHL;
use crate::opcodes::LoadRegHLRegSPr8;
use crate::opcodes::LoadRegSPRegHL;
use crate::opcodes::OrRegA;
use crate::opcodes::Pop;
use crate::opcodes::Push;
use crate::opcodes::RelJump;
use crate::opcodes::Restart;
use crate::opcodes::Return;
use crate::opcodes::RotateLeft;
use crate::opcodes::RotateRight;
use crate::opcodes::RotateRightA;
use crate::opcodes::SubRegA;
use crate::opcodes::XorRegA;
use crate::opcodes::CCF;
use crate::opcodes::SCF;

use crate::area::{Bits16, Flag};
use crate::consts::{
    DI_INSTRUCTION, EI_INSTRUCTION, HALT_INSTRUCTION, NOP_INSTRUCTION, PREFIX_CB_INSTRUCTIONS,
    STOP_INSTRUCTION,
};
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
    halt: bool,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            registers: <Registers as NewRegisters>::new(),
            interrupts: false,
            halt: false,
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
        } else if let Ok(operation) = RotateRight::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else {
            println!("This prefix_cb {:?} is not implemented!", opcode);
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

        if opcode == PREFIX_CB_INSTRUCTIONS {
            self.prefix_cb().await;
        } else if opcode == NOP_INSTRUCTION {
            dbg!("NOP Instruction does nothing");
        } else if opcode == STOP_INSTRUCTION {
            dbg!("STOP Instruction i dont know yet what to do..");
        } else if opcode == HALT_INSTRUCTION {
            dbg!("HALT Instruction... Wait until an interrupt occurs");
            self.halt = true;
        } else if opcode == DI_INSTRUCTION {
            dbg!("Disable Interrupt Instruction");
            self.interrupts = false;
        } else if opcode == EI_INSTRUCTION {
            dbg!("Enable Interrupt Instruction");
            self.interrupts = true;
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
        } else if let Ok(operation) = SubRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadRegAMemC::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadMemCRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadMem8bRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadRegAMem8b::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = DecRegNN::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else if let Ok(operation) = IncRegNN::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else if let Ok(operation) = LoadRegAHLP::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadRegAHLM::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadBCDERegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadHLMRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadHLPRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadMem16bRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadRegAMem16b::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = AndRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = OrRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = XorRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = IncRegN::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = DecRegN::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = AddRegSP::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = AddRegHL::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else if let Ok(operation) = CCF::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else if let Ok(operation) = SCF::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else if let Ok(operation) = DecAdjustRegA::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else if let Ok(operation) = LoadRegHLRegSPr8::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadRegSPRegHL::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else if let Ok(operation) = LoadMem16bRegSP::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = CmpRegA::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadR8b::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadHL8b::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadR1R2::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else if let Ok(operation) = LoadHLReg::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadRegHL::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadRR16b::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = LoadRegABCDE::try_from_primitive(opcode) {
            operation.exec(self.registers, self.memory).await;
        } else if let Ok(operation) = CPLRegA::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else if let Ok(operation) = RotateRightA::try_from_primitive(opcode) {
            operation.exec(self.registers).await;
        } else {
            println!("Not implemented!");
        }
        8
    }
}
