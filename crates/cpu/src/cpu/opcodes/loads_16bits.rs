use super::super::area::{Bits16, Bits8};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};

///1. LD n,nn
/// Description:
///  Put value nn into n.
/// Use with:
///  n = BC,DE,HL,SP
///  nn = 16 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD BC,nn 01 12
///  LD DE,nn 11 12
///  LD HL,nn 21 12
///  LD SP,nn 31 12

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum LoadNum16Num8 {
    BCNN = 0x01,
    DENN = 0x11,
    HLNN = 0x21,
    SPNN = 0x31,
}

impl<'a> LoadNum16Num8 {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let result = match self {
                LoadNum16Num8::BCNN => memory.set(byte, registers.get(Bits16::BC)),
                LoadNum16Num8::DENN => memory.set(byte, registers.get(Bits16::DE)),
                LoadNum16Num8::HLNN => memory.set(byte, registers.get(Bits16::HL)),
                LoadNum16Num8::SPNN => memory.set(byte, registers.get(Bits16::SP)),
            };
            match result {
                Ok(_) => Ok(8),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 2. LD SP,HL
/// Description:
///  Put HL into Stack Pointer (SP).
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD SP,HL F9 8

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum LoadRegSPRegHL {
    SPHL = 0xf9,
}

impl<'a> LoadRegSPRegHL {
    pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
        Ok(registers.set(Bits16::SP, registers.get(Bits16::HL)))
    }
}

///3. LD HL,SP+n
/// Description: Same as: LDHL SP,n.
/// 4. LDHL SP,n
/// Description:
///  Put SP + n effective address into HL.
/// Use with:
///  n = one byte signed immediate value.
/// Flags affected:
///  Z - Reset.
///  N - Reset.
///  H - Set or reset according to operation.
///  C - Set or reset according to operation.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LDHL SP,n F8 12

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum LoadRegHLRegSPpN {
    LDHL = 0xf8,
}

impl<'a> LoadRegHLRegSPpN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        let Ok(byte) = registers.pc.next(memory);
        Ok(registers.set(Bits16::HL, registers.get(Bits16::SP + byte)))
    }
}

/// 5. LD (nn),SP
/// Description:
///  Put Stack Pointer (SP) at address n.
/// Use with:
///  nn = two byte immediate address.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD (nn),SP 08 20

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum LoadNNRegSP {
    NNSP = 0x08,
}

impl<'a> LoadNNRegSP {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        let Ok(byte) = registers.pc.next(memory);
        Ok(memory.set(byte, registers.get(Bits16::SP)))
    }
}

/// 6. PUSH nn
/// Description:
///  Push register pair nn onto stack.
///  Decrement Stack Pointer (SP) twice.
/// Use with:
///  nn = AF,BC,DE,HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  PUSH AF F5 16
///  PUSH BC C5 16
///  PUSH DE D5 16
///  PUSH HL E5 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum PushNNRegDecSP {
    AF = 0xf5,
    BC = 0xc5,
    DE = 0xd5,
    HL = 0xe5,
}

impl<'a> PushNNRegDecSP {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let result = match self {
                // todo!() maybe need fct pop and push
                PushNNRegDecSP::AF => registers.set(Bits16::AF, byte),
                PushNNRegDecSP::BC => registers.set(Bits16::BC, byte),
                PushNNRegDecSP::DE => registers.set(Bits16::DE, byte),
                PushNNRegDecSP::HL => registers.set(Bits16::HL, byte),
            };
            match result {
                Ok(_) => Ok(8),
                Err(e) => Err(e),
            }
            Ok(Bits16::SP = Bits16::SP - 2)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

///7. POP nn
/// Description:
///  Pop two bytes off stack into register pair nn.
///  Increment Stack Pointer (SP) twice.
/// Use with:
///  nn = AF,BC,DE,HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  POP AF F1 12
///  POP BC C1 12
///  POP DE D1 12
///  POP HL E1 12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum PopNNRegIncSP {
    AF = 0xf1,
    BC = 0xc1,
    DE = 0xd1,
    HL = 0xe1,
}

impl<'a> PopNNRegIncSP {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let result = match self {
                PopNNRegIncSP::AF => memory.set(byte, registers.get(Bits16::AF)),
                PopNNRegIncSP::BC => memory.set(byte, registers.get(Bits16::BC)),
                PopNNRegIncSP::DE => memory.set(byte, registers.get(Bits16::DE)),
                PopNNRegIncSP::HL => memory.set(byte, registers.get(Bits16::HL)),
            };
            match result {
                Ok(_) => Ok(8),
                Err(e) => Err(e),
            }
            Ok(Bits16::SP = Bits16::SP + 2)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}
