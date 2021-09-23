use super::super::area::{Bits16, Bits8};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};

/// 1. CALL nn
/// Description:
///  Push address of next instruction onto stack and then
///  jump to address nn.
/// Use with:
///  nn = two byte immediate value. (LS byte first.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  CALL nn CD 12
pub enum CallNN {
    CALLNN = 0xcd,
}

/// 2. CALL cc,nn
/// Description:
///  Call address n if following condition is true:
///  cc = NZ, Call if Z flag is reset.
///  cc = Z, Call if Z flag is set.
///  cc = NC, Call if C flag is reset.
///  cc = C, Call if C flag is set.
/// Use with:
///  nn = two byte immediate value. (LS byte first.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// CALL NZ,nn C4 12
///  CALL Z,nn CC 12
///  CALL NC,nn D4 12
///  CALL C,nn DC 12
pub enum CallNIf {
    NZnn = 0xc4,
    Znn = 0xcc,
    NCnn = 0xd4,
    Cnn = 0xdc,
}