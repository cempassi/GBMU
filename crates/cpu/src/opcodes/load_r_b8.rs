use crate::area::Bits8;
use crate::pc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// 1. LD nn,n
/// Description:
///  Put value nn into n.
/// Use with:
///  nn = B,C,D,E,H,L,BC,DE,HL,SP
///  n = 8 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD B,n 06 8
///  LD C,n 0E 8
///  LD D,n 16 8
///  LD E,n 1E 8
///  LD H,n 26 8
///  LD L,n 2E 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadR8b {
    B = 0x06,
    C = 0x0E,
    D = 0x16,
    E = 0x1E,
    H = 0x26,
    L = 0x2E,
}

impl LoadR8b {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let byte = registers.borrow_mut().pc.next(memory).unwrap();
        let bits = match self {
            LoadR8b::B => Bits8::B,
            LoadR8b::C => Bits8::C,
            LoadR8b::D => Bits8::D,
            LoadR8b::E => Bits8::E,
            LoadR8b::H => Bits8::H,
            LoadR8b::L => Bits8::L,
        };
        registers.borrow_mut().set(bits, byte)
    }
}
