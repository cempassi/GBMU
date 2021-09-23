use super::super::area::{Bits16, Bits8, Flag};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};
use crate::cpu::flags::Flags;

/// 1. BIT b,r
/// Description:
///  Test bit b in register r.
/// Use with:
///  b = 0 - 7, r = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if bit b of register r is 0.
///  N - Reset.
///  H - Set.
///  C - Not affected.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// BIT b,A CB47 8
///  BIT b,B CB40 8
/// BIT b,C CB41 8
/// BIT b,D CB42 8
/// BIT b,E CB43 8
/// BIT b,H CB44 8
/// BIT b,L CB45 8
/// BIT b,(HL) CB46 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum TestBitReg {
    A = 0xcb47,
    B = 0xcb40,
    C = 0xcb41,
    D = 0xcb42,
    E = 0xcb43,
    H = 0xcb44,
    L = 0xcb45,
    // HL = 0xcb46,
}

impl<'a> TestBitReg {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                TestBitReg::A => {}
                TestBitReg::B => {}
                TestBitReg::C => {}
                TestBitReg::D => {}
                TestBitReg::E => {}
                TestBitReg::H => {}
                TestBitReg::L => {}
            };
        }
    }
}

/// 2. SET b,r
/// Description:
///  Set bit b in register r.
/// Use with:
///  b = 0 - 7, r = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  SET b,A CBC7 8
///  SET b,B CBC0 8
///  SET b,C CBC1 8
///  SET b,D CBC2 8
///  SET b,E CBC3 8
///  SET b,H CBC4 8
///  SET b,L CBC5 8
///  SET b,(HL) CBC6 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum SetBitReg {
    A = 0xcbc7,
    B = 0xcbc0,
    C = 0xcbc1,
    D = 0xcbc2,
    E = 0xcbc3,
    H = 0xcbc4,
    L = 0xcbc5,
    // HL = 0xcbc6,
}

impl<'a> SetBitReg {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                SetBitReg::A => {}
                SetBitReg::B => {}
                SetBitReg::C => {}
                SetBitReg::D => {}
                SetBitReg::E => {}
                SetBitReg::H => {}
                SetBitReg::L => {}
            };
        }
    }
}


/// 3. RES b,r
/// Description:
///  Reset bit b in register r.
/// Use with:
///  b = 0 - 7, r = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RES b,A CB87 8
///  RES b,B CB80 8
///  RES b,C CB81 8
///  RES b,D CB82 8
///  RES b,E CB83 8
///  RES b,H CB84 8
///  RES b,L CB85 8
///  RES b,(HL) CB86 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ResetBitReg {
    A = 0xcb87,
    B = 0xcb80,
    C = 0xcb81,
    D = 0xcb82,
    E = 0xcb83,
    H = 0xcb84,
    L = 0xcb85,
    // HL = 0xcb86,
}

impl<'a> ResetBitReg {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                ResetBitReg::A => {}
                ResetBitReg::B => {}
                ResetBitReg::C => {}
                ResetBitReg::D => {}
                ResetBitReg::E => {}
                ResetBitReg::H => {}
                ResetBitReg::L => {}
            };
        }
    }
}
