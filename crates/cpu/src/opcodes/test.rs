use super::consts;
use crate::futures::Set;
use crate::registers::{Bits8, Bitwise};
use crate::Cpu;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;
use std::fmt;

use super::decode::{Decode, Decoder};

/// BIT n, r8 | [HL]
/// Test bit n in register r8 or at address [HL], set the zero flag if bit not set.
///
/// Cycles: 8 | 12
///
/// Bytes: 2
///
/// Flags:
///
/// Z - Set if the selected bit is 0.
/// N - Unset
/// H - Set
/// C - Unused

#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Test {
    BBit0 = 0x40,
    BBit1 = 0x48,
    BBit2 = 0x50,
    BBit3 = 0x58,
    BBit4 = 0x60,
    BBit5 = 0x68,
    BBit6 = 0x70,
    BBit7 = 0x78,
    CBit0 = 0x41,
    CBit1 = 0x49,
    CBit2 = 0x51,
    CBit3 = 0x59,
    CBit4 = 0x61,
    CBit5 = 0x69,
    CBit6 = 0x71,
    CBit7 = 0x79,
    DBit0 = 0x42,
    DBit1 = 0x4A,
    DBit2 = 0x52,
    DBit3 = 0x5A,
    DBit4 = 0x62,
    DBit5 = 0x6A,
    DBit6 = 0x72,
    DBit7 = 0x7A,
    EBit0 = 0x43,
    EBit1 = 0x4B,
    EBit2 = 0x53,
    EBit3 = 0x5B,
    EBit4 = 0x63,
    EBit5 = 0x6B,
    EBit6 = 0x73,
    EBit7 = 0x7B,
    HBit0 = 0x44,
    HBit1 = 0x4C,
    HBit2 = 0x54,
    HBit3 = 0x5C,
    HBit4 = 0x64,
    HBit5 = 0x6C,
    HBit6 = 0x74,
    HBit7 = 0x7C,
    LBit0 = 0x45,
    LBit1 = 0x4D,
    LBit2 = 0x55,
    LBit3 = 0x5D,
    LBit4 = 0x65,
    LBit5 = 0x6D,
    LBit6 = 0x75,
    LBit7 = 0x7D,
    HLBit0 = 0x46,
    HLBit1 = 0x4E,
    HLBit2 = 0x56,
    HLBit3 = 0x5E,
    HLBit4 = 0x66,
    HLBit5 = 0x6E,
    HLBit6 = 0x76,
    HLBit7 = 0x7E,
    ABit0 = 0x47,
    ABit1 = 0x4F,
    ABit2 = 0x57,
    ABit3 = 0x5F,
    ABit4 = 0x67,
    ABit5 = 0x6F,
    ABit6 = 0x77,
    ABit7 = 0x7F,
}

impl Decoder for Test {
    fn decode(self, cpu: Cpu) -> Decode {
        Box::pin(self.exec(cpu))
    }
}

impl Test {
    pub async fn exec(self, cpu: Cpu) -> Result<u8, Error> {
        let cycles = match self {
            Test::BBit0 => cpu.borrow_mut().registers.test(Bits8::B, consts::BIT0),
            Test::BBit1 => cpu.borrow_mut().registers.test(Bits8::B, consts::BIT1),
            Test::BBit2 => cpu.borrow_mut().registers.test(Bits8::B, consts::BIT2),
            Test::BBit3 => cpu.borrow_mut().registers.test(Bits8::B, consts::BIT3),
            Test::BBit4 => cpu.borrow_mut().registers.test(Bits8::B, consts::BIT4),
            Test::BBit5 => cpu.borrow_mut().registers.test(Bits8::B, consts::BIT5),
            Test::BBit6 => cpu.borrow_mut().registers.test(Bits8::B, consts::BIT6),
            Test::BBit7 => cpu.borrow_mut().registers.test(Bits8::B, consts::BIT7),
            Test::CBit0 => cpu.borrow_mut().registers.test(Bits8::C, consts::BIT0),
            Test::CBit1 => cpu.borrow_mut().registers.test(Bits8::C, consts::BIT1),
            Test::CBit2 => cpu.borrow_mut().registers.test(Bits8::C, consts::BIT2),
            Test::CBit3 => cpu.borrow_mut().registers.test(Bits8::C, consts::BIT3),
            Test::CBit4 => cpu.borrow_mut().registers.test(Bits8::C, consts::BIT4),
            Test::CBit5 => cpu.borrow_mut().registers.test(Bits8::C, consts::BIT5),
            Test::CBit6 => cpu.borrow_mut().registers.test(Bits8::C, consts::BIT6),
            Test::CBit7 => cpu.borrow_mut().registers.test(Bits8::C, consts::BIT7),
            Test::DBit0 => cpu.borrow_mut().registers.test(Bits8::D, consts::BIT0),
            Test::DBit1 => cpu.borrow_mut().registers.test(Bits8::D, consts::BIT1),
            Test::DBit2 => cpu.borrow_mut().registers.test(Bits8::D, consts::BIT2),
            Test::DBit3 => cpu.borrow_mut().registers.test(Bits8::D, consts::BIT3),
            Test::DBit4 => cpu.borrow_mut().registers.test(Bits8::D, consts::BIT4),
            Test::DBit5 => cpu.borrow_mut().registers.test(Bits8::D, consts::BIT5),
            Test::DBit6 => cpu.borrow_mut().registers.test(Bits8::D, consts::BIT6),
            Test::DBit7 => cpu.borrow_mut().registers.test(Bits8::D, consts::BIT7),
            Test::EBit0 => cpu.borrow_mut().registers.test(Bits8::E, consts::BIT0),
            Test::EBit1 => cpu.borrow_mut().registers.test(Bits8::E, consts::BIT1),
            Test::EBit2 => cpu.borrow_mut().registers.test(Bits8::E, consts::BIT2),
            Test::EBit3 => cpu.borrow_mut().registers.test(Bits8::E, consts::BIT3),
            Test::EBit4 => cpu.borrow_mut().registers.test(Bits8::E, consts::BIT4),
            Test::EBit5 => cpu.borrow_mut().registers.test(Bits8::E, consts::BIT5),
            Test::EBit6 => cpu.borrow_mut().registers.test(Bits8::E, consts::BIT6),
            Test::EBit7 => cpu.borrow_mut().registers.test(Bits8::E, consts::BIT7),
            Test::HBit0 => cpu.borrow_mut().registers.test(Bits8::H, consts::BIT0),
            Test::HBit1 => cpu.borrow_mut().registers.test(Bits8::H, consts::BIT1),
            Test::HBit2 => cpu.borrow_mut().registers.test(Bits8::H, consts::BIT2),
            Test::HBit3 => cpu.borrow_mut().registers.test(Bits8::H, consts::BIT3),
            Test::HBit4 => cpu.borrow_mut().registers.test(Bits8::H, consts::BIT4),
            Test::HBit5 => cpu.borrow_mut().registers.test(Bits8::H, consts::BIT5),
            Test::HBit6 => cpu.borrow_mut().registers.test(Bits8::H, consts::BIT6),
            Test::HBit7 => cpu.borrow_mut().registers.test(Bits8::H, consts::BIT7),
            Test::LBit0 => cpu.borrow_mut().registers.test(Bits8::L, consts::BIT0),
            Test::LBit1 => cpu.borrow_mut().registers.test(Bits8::L, consts::BIT1),
            Test::LBit2 => cpu.borrow_mut().registers.test(Bits8::L, consts::BIT2),
            Test::LBit3 => cpu.borrow_mut().registers.test(Bits8::L, consts::BIT3),
            Test::LBit4 => cpu.borrow_mut().registers.test(Bits8::L, consts::BIT4),
            Test::LBit5 => cpu.borrow_mut().registers.test(Bits8::L, consts::BIT5),
            Test::LBit6 => cpu.borrow_mut().registers.test(Bits8::L, consts::BIT6),
            Test::LBit7 => cpu.borrow_mut().registers.test(Bits8::L, consts::BIT7),
            Test::ABit0 => cpu.borrow_mut().registers.test(Bits8::A, consts::BIT0),
            Test::ABit1 => cpu.borrow_mut().registers.test(Bits8::A, consts::BIT1),
            Test::ABit2 => cpu.borrow_mut().registers.test(Bits8::A, consts::BIT2),
            Test::ABit3 => cpu.borrow_mut().registers.test(Bits8::A, consts::BIT3),
            Test::ABit4 => cpu.borrow_mut().registers.test(Bits8::A, consts::BIT4),
            Test::ABit5 => cpu.borrow_mut().registers.test(Bits8::A, consts::BIT5),
            Test::ABit6 => cpu.borrow_mut().registers.test(Bits8::A, consts::BIT6),
            Test::ABit7 => cpu.borrow_mut().registers.test(Bits8::A, consts::BIT7),
            Test::HLBit0 => Set::TestHL(consts::BIT0).run(cpu).await?,
            Test::HLBit1 => Set::TestHL(consts::BIT1).run(cpu).await?,
            Test::HLBit2 => Set::TestHL(consts::BIT2).run(cpu).await?,
            Test::HLBit3 => Set::TestHL(consts::BIT3).run(cpu).await?,
            Test::HLBit4 => Set::TestHL(consts::BIT4).run(cpu).await?,
            Test::HLBit5 => Set::TestHL(consts::BIT5).run(cpu).await?,
            Test::HLBit6 => Set::TestHL(consts::BIT6).run(cpu).await?,
            Test::HLBit7 => Set::TestHL(consts::BIT7).run(cpu).await?,
        };
        Ok(cycles)
    }
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Test::BBit0 => write!(f, "TEST B, Bit 0"),
            Test::BBit1 => write!(f, "TEST B, Bit 1"),
            Test::BBit2 => write!(f, "TEST B, Bit 2"),
            Test::BBit3 => write!(f, "TEST B, Bit 3"),
            Test::BBit4 => write!(f, "TEST B, Bit 4"),
            Test::BBit5 => write!(f, "TEST B, Bit 5"),
            Test::BBit6 => write!(f, "TEST B, Bit 6"),
            Test::BBit7 => write!(f, "TEST B, Bit 7"),
            Test::CBit0 => write!(f, "TEST C, Bit 0"),
            Test::CBit1 => write!(f, "TEST C, Bit 1"),
            Test::CBit2 => write!(f, "TEST C, Bit 2"),
            Test::CBit3 => write!(f, "TEST C, Bit 3"),
            Test::CBit4 => write!(f, "TEST C, Bit 4"),
            Test::CBit5 => write!(f, "TEST C, Bit 5"),
            Test::CBit6 => write!(f, "TEST C, Bit 6"),
            Test::CBit7 => write!(f, "TEST C, Bit 7"),
            Test::DBit0 => write!(f, "TEST D, Bit 0"),
            Test::DBit1 => write!(f, "TEST D, Bit 1"),
            Test::DBit2 => write!(f, "TEST D, Bit 2"),
            Test::DBit3 => write!(f, "TEST D, Bit 3"),
            Test::DBit4 => write!(f, "TEST D, Bit 4"),
            Test::DBit5 => write!(f, "TEST D, Bit 5"),
            Test::DBit6 => write!(f, "TEST D, Bit 6"),
            Test::DBit7 => write!(f, "TEST D, Bit 7"),
            Test::EBit0 => write!(f, "TEST E, Bit 0"),
            Test::EBit1 => write!(f, "TEST E, Bit 1"),
            Test::EBit2 => write!(f, "TEST E, Bit 2"),
            Test::EBit3 => write!(f, "TEST E, Bit 3"),
            Test::EBit4 => write!(f, "TEST E, Bit 4"),
            Test::EBit5 => write!(f, "TEST E, Bit 5"),
            Test::EBit6 => write!(f, "TEST E, Bit 6"),
            Test::EBit7 => write!(f, "TEST E, Bit 7"),
            Test::HBit0 => write!(f, "TEST H, Bit 0"),
            Test::HBit1 => write!(f, "TEST H, Bit 1"),
            Test::HBit2 => write!(f, "TEST H, Bit 2"),
            Test::HBit3 => write!(f, "TEST H, Bit 3"),
            Test::HBit4 => write!(f, "TEST H, Bit 4"),
            Test::HBit5 => write!(f, "TEST H, Bit 5"),
            Test::HBit6 => write!(f, "TEST H, Bit 6"),
            Test::HBit7 => write!(f, "TEST H, Bit 7"),
            Test::LBit0 => write!(f, "TEST L, Bit 0"),
            Test::LBit1 => write!(f, "TEST L, Bit 1"),
            Test::LBit2 => write!(f, "TEST L, Bit 2"),
            Test::LBit3 => write!(f, "TEST L, Bit 3"),
            Test::LBit4 => write!(f, "TEST L, Bit 4"),
            Test::LBit5 => write!(f, "TEST L, Bit 5"),
            Test::LBit6 => write!(f, "TEST L, Bit 6"),
            Test::LBit7 => write!(f, "TEST L, Bit 7"),
            Test::HLBit0 => write!(f, "TEST [HL], Bit 0"),
            Test::HLBit1 => write!(f, "TEST [HL], Bit 1"),
            Test::HLBit2 => write!(f, "TEST [HL], Bit 2"),
            Test::HLBit3 => write!(f, "TEST [HL], Bit 3"),
            Test::HLBit4 => write!(f, "TEST [HL], Bit 4"),
            Test::HLBit5 => write!(f, "TEST [HL], Bit 5"),
            Test::HLBit6 => write!(f, "TEST [HL], Bit 6"),
            Test::HLBit7 => write!(f, "TEST [HL], Bit 7"),
            Test::ABit0 => write!(f, "TEST A, Bit 0"),
            Test::ABit1 => write!(f, "TEST A, Bit 1"),
            Test::ABit2 => write!(f, "TEST A, Bit 2"),
            Test::ABit3 => write!(f, "TEST A, Bit 3"),
            Test::ABit4 => write!(f, "TEST A, Bit 4"),
            Test::ABit5 => write!(f, "TEST A, Bit 5"),
            Test::ABit6 => write!(f, "TEST A, Bit 6"),
            Test::ABit7 => write!(f, "TEST A, Bit 7"),
        }
    }
}

#[cfg(test)]
mod test_test_bit {
    use super::Test;
    use crate::registers::{Bits16, Bits8, Bus, Flag};
    use crate::{Access, Cpu};
    use shared::execute;

    #[test]
    fn test_if_bit_6_in_register_b_is_not_zero() {
        let src = 0b0100_0000;
        let expected = false;
        let cpu = Cpu::default();
        let instruction = Test::BBit6;
        cpu.borrow_mut().registers.set(Bits8::B, src);

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let is_zero = cpu.borrow().registers.get(Flag::Z);
        assert!(is_zero == expected);
    }

    #[test]
    fn test_if_bit_4_in_register_d_is_zero() {
        let expected = true;
        let cpu = Cpu::default();
        let instruction = Test::DBit4;

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let is_zero = cpu.borrow().registers.get(Flag::Z);
        assert!(is_zero == expected);
    }

    #[test]
    fn test_if_bit_2_in_hl_is_not_zero() {
        let hl = 0xc008;
        let src = 0b0000_0100;
        let expected = false;
        let cpu = Cpu::default();
        let instruction = Test::HLBit2;
        cpu.borrow_mut().registers.set(Bits16::HL, hl);
        cpu.memory().borrow_mut().set_u8(hl, src).unwrap();

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let is_zero = cpu.borrow().registers.get(Flag::Z);
        assert!(is_zero == expected);
    }
}
