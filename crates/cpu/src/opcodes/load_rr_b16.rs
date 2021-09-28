use crate::area::Bits16;
use crate::pc::NextPc;
use crate::{RegisterBus, Registers};
use memory::Memory;
use num_enum::TryFromPrimitive;

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
pub enum LoadRR16b {
    BC = 0x01,
    DE = 0x11,
    HL = 0x21,
    SP = 0x31,
}

impl LoadRR16b {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let first_byte = registers.borrow_mut().pc.next(memory.clone()).unwrap();
        let second_byte = registers.borrow_mut().pc.next(memory).unwrap();
        let data = (second_byte as u16) << 8 | (first_byte as u16) as u16;
        let dst = match self {
            LoadRR16b::BC => Bits16::BC,
            LoadRR16b::DE => Bits16::DE,
            LoadRR16b::HL => Bits16::HL,
            LoadRR16b::SP => Bits16::SP,
        };
        registers.borrow_mut().set(dst, data)
    }
}
