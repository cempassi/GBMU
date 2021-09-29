use crate::area::{Bits8, Flag};
use crate::{RegisterBus, Registers};
use num_enum::TryFromPrimitive;

///10. DEC n
/// Description:
///  Decrement register n.
/// Use with:
///  n = A,B,C,D,E,H,L
/// Flags affected:
///  Z - Set if result is zero.
///  N - Set.
///  H - Set if borrow from bit 4.
///  C - Not changed
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  DEC A 3D 4
///  DEC B 05 4
///  DEC C 0D 4
///  DEC D 15 4
///  DEC E 1D 4
///  DEC H 25 4
///  DEC L 2D 4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DecR {
    A = 0x3d,
    B = 0x05,
    C = 0x0d,
    D = 0x15,
    E = 0x1d,
    H = 0x25,
    L = 0x2d,
}

impl DecR {
    pub fn exec(self, registers: Registers) {
        let reg = match self {
            DecR::A => Bits8::A,
            DecR::B => Bits8::B,
            DecR::C => Bits8::C,
            DecR::D => Bits8::D,
            DecR::E => Bits8::E,
            DecR::H => Bits8::H,
            DecR::L => Bits8::L,
        };
        let nbr = { registers.borrow_mut().get(reg).wrapping_sub(1) };
        registers.borrow_mut().set(Flag::N, true);
        if nbr == 0 {
            registers.borrow_mut().set(Flag::Z, true)
        }
        if nbr == 15 {
            registers.borrow_mut().set(Flag::H, true)
        }
        registers.borrow_mut().set(reg, nbr);
    }
}
