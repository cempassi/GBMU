use super::super::area::{Bits16, Bits8};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};

/// 1. JP nn
/// Description:
///  Jump to address nn.
/// Use with:
///  nn = two byte immediate value. (LS byte first.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  JP nn C3 12
pub enum JumpNN {
    JPNN = 0xc3,
}

/// 2. JP cc,nn
/// Description:
///  Jump to address n if following condition is true:
///  cc = NZ, Jump if Z flag is reset.
///  cc = Z, Jump if Z flag is set.
///  cc = NC, Jump if C flag is reset.
///  cc = C, Jump if C flag is set.
/// Use with:
///  nn = two byte immediate value. (LS byte first.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  JP NZ,nn C2 12
///  JP Z,nn CA 12
///  JP NC,nn D2 12
///  JP C,nn DA 12
pub enum JumpNIf {
    JPNZnn = 0xc2,
    JPZnn = 0xcA,
    JPNCnn = 0xd2,
    JPCnn = 0xda,
}

/// 3. JP (HL)
/// Description:
///  Jump to address contained in HL.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  JP (HL) E9 4
pub enum JumpHL {
    JPHL = 0xe9,
}

/// 4. JR n
/// Description:
///  Add n to current address and jump to it.
/// Use with:
///  n = one byte signed immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  JR n 18 8
pub enum AddNJump {
    JR = 0x19,
}

/// 5. JR cc,n
/// Description:
///  If following condition is true then add n to current
///  address and jump to it:
/// Use with:
///  n = one byte signed immediate value
///  cc = NZ, Jump if Z flag is reset.
///  cc = Z, Jump if Z flag is set.
///  cc = NC, Jump if C flag is reset.
///  cc = C, Jump if C flag is set.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  JR NZ,* 20 8
///  JR Z,* 28 8
///  JR NC,* 30 8
///  JR C,* 38 8
pub enum AddNJumpIf {
    JRNZ = 0x20,
    JRZ = 0x28 ,
    JRNC = 0x30,
    JRC = 0x38 ,
}
