use super::super::area::{Bits16, Bits8, Flag};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};
use crate::cpu::flags::Flags;

///1. ADD HL,n
/// Description:
///  Add n to HL.
/// Use with:
///  n = BC,DE,HL,SP
/// Flags affected:
///  Z - Not affected.
///  N - Reset.
///  H - Set if carry from bit 11.
///  C - Set if carry from bit 15.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  ADD HL,BC 09 8
///  ADD HL,DE 19 8
///  ADD HL,HL 29 8
///  ADD HL,SP 39 8

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum AddRegHLNum16bit {
    HLBC = 0x09,
    HLDE = 0x19,
    HLHL = 0x29,
    HLSP = 0x39,
}

impl<'a> AddRegHLNum16bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                AddRegHLNum16bit::HLBC => Bits16::BC,
                AddRegHLNum16bit::HLDE => Bits16::DE,
                AddRegHLNum16bit::HLHL => Bits16::HL,
                AddRegHLNum16bit::HLSP => Bits16::SP,
            };
            let result = registers.get(bits) + registers.get(Bits16::HL);
            registers.set(Bits16::HL, result);
            Ok()
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 2. ADD SP,n
/// Description:
///  Add n to Stack Pointer (SP).
/// Use with:
///  n = one byte signed immediate value (#).
/// Flags affected:
///  Z - Reset.
///  N - Reset.
///  H - Set or reset according to operation.
///  C - Set or reset according to operation.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  ADD SP,# E8 16

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum AddRegSPNum16bit {
    SPN = 0xe8,
}

impl<'a> AddCRegANum16bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let result = registers.get(Bits16::SP) + byte;
            registers.set(Bits16::SP, result)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 3. INC nn
/// Description:
///  Increment register nn.
/// Use with:
///  nn = BC,DE,HL,SP
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  INC BC 03 8
///  INC DE 13 8
///  INC HL 23 8
///  INC SP 33 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum IncRegNN {
    BC = 0x03,
    DE = 0x13,
    HL = 0x23,
    SP = 0x33,
}

impl<'a> IncRegNN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let result = match self {
              IncRegNN::BC => Bits16::BC,
              IncRegNN::DE => Bits16::DE,
              IncRegNN::HL => Bits16::HL,
              IncRegNN::SP => Bits16::SP,
            };
            let nbr =  registers.get(result) + 1;
            register.set(result, nbr);
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 4. DEC nn
/// Description:
///  Decrement register nn.
/// Use with:
///  nn = BC,DE,HL,SP
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  DEC BC 0B 8
///  DEC DE 1B 8
///  DEC HL 2B 8
///  DEC SP 3B 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum DecRegNN {
    BC = 0x0b,
    DE = 0x1b,
    HL = 0x2b,
    SP = 0x3b,
}

impl<'a> DecRegNN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                DecRegNN::BC => Bits16::BC,
                DecRegNN::DE => Bits16::DE,
                DecRegNN::HL => Bits16::HL,
                DecRegNN::SP => Bits16::SP,
            };
            let nbr = registers.get(bits) - 1;
            registers.set(bits, nbr);
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}
