use super::super::area::{Bits16, Bits8};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};

/// 1. RET
/// Description:
///  Pop two bytes from stack & jump to that address.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RET -/- C9 8
pub enum Return {
    RET = 0xc9,
}

/// 2. RET cc
/// Description:
///  Return if following condition is true:
/// Use with:
///  cc = NZ, Return if Z flag is reset.
///  cc = Z, Return if Z flag is set.
///  cc = NC, Return if C flag is reset.
///  cc = C, Return if C flag is set.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RET NZ C0 8
///  RET Z C8 8
///  RET NC D0 8
///  RET C D8 8
pub enum ReturnIf {
    RETNZ = 0xc0,
    RETZ = 0xc8,
    RETNC = 0xd0,
    RETC = 0xd8,
}

/// 3. RETI
/// Description:
///  Pop two bytes from stack & jump to that address then
///  enable interrupts.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RETI -/- D9 8
pub enum ReturnInterrupts {
    RETI = 0xd9,
}