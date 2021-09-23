use super::super::area::{Bits16, Bits8};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};

pub const Z: u8 = 0x80;
pub const N: u8 = 0x40;
pub const H: u8 = 0x20;
pub const C: u8 = 0x10;


///1. RST n
/// Description:
///  Push present address onto stack.
///  Jump to address $0000 + n.
/// Use with:
///  n = $00,$08,$10,$18,$20,$28,$30,$38
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RST 00H C7 32
///  RST 08H CF 32
///  RST 10H D7 32
///  RST 18H DF 32
///  RST 20H E7 32
///  RST 28H EF 32
///  RST 30H F7 32
///  RST 38H FF 32
pub enum RestartN {
    H00 =  0xc7,
    H08 =  0xcf,
    H10 =  0xd7,
    H18 =  0xdf,
    H20 =  0xe7,
    H28 =  0xef,
    H30 =  0xf7,
    H38 =  0xff,
}

