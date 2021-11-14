use super::consts;
use crate::futures::{CbOperation as Operation, Set};
use crate::registers::{Bits8, Bitwise};
use crate::Cpu;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;
use std::fmt;

use super::decode::{Decode, Decoder};

/// RESET n, r8 | [HL]
/// Set bit n in register r8 or at address [HL] to 0.
///
/// Cycles: 8 | 16
///
/// Bytes: 2
///
/// Flags: None affected.

#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Bitset {
    BBit0 = 0xC0,
    BBit1 = 0xC8,
    BBit2 = 0xD0,
    BBit3 = 0xD8,
    BBit4 = 0xE0,
    BBit5 = 0xE8,
    BBit6 = 0xF0,
    BBit7 = 0xF8,
    CBit0 = 0xC1,
    CBit1 = 0xC9,
    CBit2 = 0xD1,
    CBit3 = 0xD9,
    CBit4 = 0xE1,
    CBit5 = 0xE9,
    CBit6 = 0xF1,
    CBit7 = 0xF9,
    DBit0 = 0xC2,
    DBit1 = 0xCA,
    DBit2 = 0xD2,
    DBit3 = 0xDA,
    DBit4 = 0xE2,
    DBit5 = 0xEA,
    DBit6 = 0xF2,
    DBit7 = 0xFA,
    EBit0 = 0xC3,
    EBit1 = 0xCB,
    EBit2 = 0xD3,
    EBit3 = 0xDB,
    EBit4 = 0xE3,
    EBit5 = 0xEB,
    EBit6 = 0xF3,
    EBit7 = 0xFB,
    HBit0 = 0xC4,
    HBit1 = 0xCC,
    HBit2 = 0xD4,
    HBit3 = 0xDC,
    HBit4 = 0xE4,
    HBit5 = 0xEC,
    HBit6 = 0xF4,
    HBit7 = 0xFC,
    LBit0 = 0xC5,
    LBit1 = 0xCD,
    LBit2 = 0xD5,
    LBit3 = 0xDD,
    LBit4 = 0xE5,
    LBit5 = 0xED,
    LBit6 = 0xF5,
    LBit7 = 0xFD,
    HLBit0 = 0xC6,
    HLBit1 = 0xCE,
    HLBit2 = 0xD6,
    HLBit3 = 0xDE,
    HLBit4 = 0xE6,
    HLBit5 = 0xEE,
    HLBit6 = 0xF6,
    HLBit7 = 0xFE,
    ABit0 = 0xC7,
    ABit1 = 0xCF,
    ABit2 = 0xD7,
    ABit3 = 0xDF,
    ABit4 = 0xE7,
    ABit5 = 0xEF,
    ABit6 = 0xF7,
    ABit7 = 0xFF,
}

impl Decoder for Bitset {
    fn decode(self, cpu: Cpu) -> Decode {
        Box::pin(self.exec(cpu))
    }
}

impl Bitset {
    pub async fn exec(self, cpu: Cpu) -> Result<u8, Error> {
        let cycles = match self {
            Bitset::BBit0 => cpu.borrow_mut().registers.bitset(Bits8::B, consts::BIT0),
            Bitset::BBit1 => cpu.borrow_mut().registers.bitset(Bits8::B, consts::BIT1),
            Bitset::BBit2 => cpu.borrow_mut().registers.bitset(Bits8::B, consts::BIT2),
            Bitset::BBit3 => cpu.borrow_mut().registers.bitset(Bits8::B, consts::BIT3),
            Bitset::BBit4 => cpu.borrow_mut().registers.bitset(Bits8::B, consts::BIT4),
            Bitset::BBit5 => cpu.borrow_mut().registers.bitset(Bits8::B, consts::BIT5),
            Bitset::BBit6 => cpu.borrow_mut().registers.bitset(Bits8::B, consts::BIT6),
            Bitset::BBit7 => cpu.borrow_mut().registers.bitset(Bits8::B, consts::BIT7),
            Bitset::CBit0 => cpu.borrow_mut().registers.bitset(Bits8::C, consts::BIT0),
            Bitset::CBit1 => cpu.borrow_mut().registers.bitset(Bits8::C, consts::BIT1),
            Bitset::CBit2 => cpu.borrow_mut().registers.bitset(Bits8::C, consts::BIT2),
            Bitset::CBit3 => cpu.borrow_mut().registers.bitset(Bits8::C, consts::BIT3),
            Bitset::CBit4 => cpu.borrow_mut().registers.bitset(Bits8::C, consts::BIT4),
            Bitset::CBit5 => cpu.borrow_mut().registers.bitset(Bits8::C, consts::BIT5),
            Bitset::CBit6 => cpu.borrow_mut().registers.bitset(Bits8::C, consts::BIT6),
            Bitset::CBit7 => cpu.borrow_mut().registers.bitset(Bits8::C, consts::BIT7),
            Bitset::DBit0 => cpu.borrow_mut().registers.bitset(Bits8::D, consts::BIT0),
            Bitset::DBit1 => cpu.borrow_mut().registers.bitset(Bits8::D, consts::BIT1),
            Bitset::DBit2 => cpu.borrow_mut().registers.bitset(Bits8::D, consts::BIT2),
            Bitset::DBit3 => cpu.borrow_mut().registers.bitset(Bits8::D, consts::BIT3),
            Bitset::DBit4 => cpu.borrow_mut().registers.bitset(Bits8::D, consts::BIT4),
            Bitset::DBit5 => cpu.borrow_mut().registers.bitset(Bits8::D, consts::BIT5),
            Bitset::DBit6 => cpu.borrow_mut().registers.bitset(Bits8::D, consts::BIT6),
            Bitset::DBit7 => cpu.borrow_mut().registers.bitset(Bits8::D, consts::BIT7),
            Bitset::EBit0 => cpu.borrow_mut().registers.bitset(Bits8::E, consts::BIT0),
            Bitset::EBit1 => cpu.borrow_mut().registers.bitset(Bits8::E, consts::BIT1),
            Bitset::EBit2 => cpu.borrow_mut().registers.bitset(Bits8::E, consts::BIT2),
            Bitset::EBit3 => cpu.borrow_mut().registers.bitset(Bits8::E, consts::BIT3),
            Bitset::EBit4 => cpu.borrow_mut().registers.bitset(Bits8::E, consts::BIT4),
            Bitset::EBit5 => cpu.borrow_mut().registers.bitset(Bits8::E, consts::BIT5),
            Bitset::EBit6 => cpu.borrow_mut().registers.bitset(Bits8::E, consts::BIT6),
            Bitset::EBit7 => cpu.borrow_mut().registers.bitset(Bits8::E, consts::BIT7),
            Bitset::HBit0 => cpu.borrow_mut().registers.bitset(Bits8::H, consts::BIT0),
            Bitset::HBit1 => cpu.borrow_mut().registers.bitset(Bits8::H, consts::BIT1),
            Bitset::HBit2 => cpu.borrow_mut().registers.bitset(Bits8::H, consts::BIT2),
            Bitset::HBit3 => cpu.borrow_mut().registers.bitset(Bits8::H, consts::BIT3),
            Bitset::HBit4 => cpu.borrow_mut().registers.bitset(Bits8::H, consts::BIT4),
            Bitset::HBit5 => cpu.borrow_mut().registers.bitset(Bits8::H, consts::BIT5),
            Bitset::HBit6 => cpu.borrow_mut().registers.bitset(Bits8::H, consts::BIT6),
            Bitset::HBit7 => cpu.borrow_mut().registers.bitset(Bits8::H, consts::BIT7),
            Bitset::LBit0 => cpu.borrow_mut().registers.bitset(Bits8::L, consts::BIT0),
            Bitset::LBit1 => cpu.borrow_mut().registers.bitset(Bits8::L, consts::BIT1),
            Bitset::LBit2 => cpu.borrow_mut().registers.bitset(Bits8::L, consts::BIT2),
            Bitset::LBit3 => cpu.borrow_mut().registers.bitset(Bits8::L, consts::BIT3),
            Bitset::LBit4 => cpu.borrow_mut().registers.bitset(Bits8::L, consts::BIT4),
            Bitset::LBit5 => cpu.borrow_mut().registers.bitset(Bits8::L, consts::BIT5),
            Bitset::LBit6 => cpu.borrow_mut().registers.bitset(Bits8::L, consts::BIT6),
            Bitset::LBit7 => cpu.borrow_mut().registers.bitset(Bits8::L, consts::BIT7),
            Bitset::ABit0 => cpu.borrow_mut().registers.bitset(Bits8::A, consts::BIT0),
            Bitset::ABit1 => cpu.borrow_mut().registers.bitset(Bits8::A, consts::BIT1),
            Bitset::ABit2 => cpu.borrow_mut().registers.bitset(Bits8::A, consts::BIT2),
            Bitset::ABit3 => cpu.borrow_mut().registers.bitset(Bits8::A, consts::BIT3),
            Bitset::ABit4 => cpu.borrow_mut().registers.bitset(Bits8::A, consts::BIT4),
            Bitset::ABit5 => cpu.borrow_mut().registers.bitset(Bits8::A, consts::BIT5),
            Bitset::ABit6 => cpu.borrow_mut().registers.bitset(Bits8::A, consts::BIT6),
            Bitset::ABit7 => cpu.borrow_mut().registers.bitset(Bits8::A, consts::BIT7),
            Bitset::HLBit0 => Set::CbHL(Operation::Bitset(consts::BIT0)).run(cpu).await?,
            Bitset::HLBit1 => Set::CbHL(Operation::Bitset(consts::BIT1)).run(cpu).await?,
            Bitset::HLBit2 => Set::CbHL(Operation::Bitset(consts::BIT2)).run(cpu).await?,
            Bitset::HLBit3 => Set::CbHL(Operation::Bitset(consts::BIT3)).run(cpu).await?,
            Bitset::HLBit4 => Set::CbHL(Operation::Bitset(consts::BIT4)).run(cpu).await?,
            Bitset::HLBit5 => Set::CbHL(Operation::Bitset(consts::BIT5)).run(cpu).await?,
            Bitset::HLBit6 => Set::CbHL(Operation::Bitset(consts::BIT6)).run(cpu).await?,
            Bitset::HLBit7 => Set::CbHL(Operation::Bitset(consts::BIT7)).run(cpu).await?,
        };
        Ok(cycles)
    }
}

impl fmt::Display for Bitset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bitset::BBit0 => write!(f, "SET B, Bit 0"),
            Bitset::BBit1 => write!(f, "SET B, Bit 1"),
            Bitset::BBit2 => write!(f, "SET B, Bit 2"),
            Bitset::BBit3 => write!(f, "SET B, Bit 3"),
            Bitset::BBit4 => write!(f, "SET B, Bit 4"),
            Bitset::BBit5 => write!(f, "SET B, Bit 5"),
            Bitset::BBit6 => write!(f, "SET B, Bit 6"),
            Bitset::BBit7 => write!(f, "SET B, Bit 7"),
            Bitset::CBit0 => write!(f, "SET C, Bit 0"),
            Bitset::CBit1 => write!(f, "SET C, Bit 1"),
            Bitset::CBit2 => write!(f, "SET C, Bit 2"),
            Bitset::CBit3 => write!(f, "SET C, Bit 3"),
            Bitset::CBit4 => write!(f, "SET C, Bit 4"),
            Bitset::CBit5 => write!(f, "SET C, Bit 5"),
            Bitset::CBit6 => write!(f, "SET C, Bit 6"),
            Bitset::CBit7 => write!(f, "SET C, Bit 7"),
            Bitset::DBit0 => write!(f, "SET D, Bit 0"),
            Bitset::DBit1 => write!(f, "SET D, Bit 1"),
            Bitset::DBit2 => write!(f, "SET D, Bit 2"),
            Bitset::DBit3 => write!(f, "SET D, Bit 3"),
            Bitset::DBit4 => write!(f, "SET D, Bit 4"),
            Bitset::DBit5 => write!(f, "SET D, Bit 5"),
            Bitset::DBit6 => write!(f, "SET D, Bit 6"),
            Bitset::DBit7 => write!(f, "SET D, Bit 7"),
            Bitset::EBit0 => write!(f, "SET E, Bit 0"),
            Bitset::EBit1 => write!(f, "SET E, Bit 1"),
            Bitset::EBit2 => write!(f, "SET E, Bit 2"),
            Bitset::EBit3 => write!(f, "SET E, Bit 3"),
            Bitset::EBit4 => write!(f, "SET E, Bit 4"),
            Bitset::EBit5 => write!(f, "SET E, Bit 5"),
            Bitset::EBit6 => write!(f, "SET E, Bit 6"),
            Bitset::EBit7 => write!(f, "SET E, Bit 7"),
            Bitset::HBit0 => write!(f, "SET H, Bit 0"),
            Bitset::HBit1 => write!(f, "SET H, Bit 1"),
            Bitset::HBit2 => write!(f, "SET H, Bit 2"),
            Bitset::HBit3 => write!(f, "SET H, Bit 3"),
            Bitset::HBit4 => write!(f, "SET H, Bit 4"),
            Bitset::HBit5 => write!(f, "SET H, Bit 5"),
            Bitset::HBit6 => write!(f, "SET H, Bit 6"),
            Bitset::HBit7 => write!(f, "SET H, Bit 7"),
            Bitset::LBit0 => write!(f, "SET L, Bit 0"),
            Bitset::LBit1 => write!(f, "SET L, Bit 1"),
            Bitset::LBit2 => write!(f, "SET L, Bit 2"),
            Bitset::LBit3 => write!(f, "SET L, Bit 3"),
            Bitset::LBit4 => write!(f, "SET L, Bit 4"),
            Bitset::LBit5 => write!(f, "SET L, Bit 5"),
            Bitset::LBit6 => write!(f, "SET L, Bit 6"),
            Bitset::LBit7 => write!(f, "SET L, Bit 7"),
            Bitset::HLBit0 => write!(f, "SET [HL], Bit 0"),
            Bitset::HLBit1 => write!(f, "SET [HL], Bit 1"),
            Bitset::HLBit2 => write!(f, "SET [HL], Bit 2"),
            Bitset::HLBit3 => write!(f, "SET [HL], Bit 3"),
            Bitset::HLBit4 => write!(f, "SET [HL], Bit 4"),
            Bitset::HLBit5 => write!(f, "SET [HL], Bit 5"),
            Bitset::HLBit6 => write!(f, "SET [HL], Bit 6"),
            Bitset::HLBit7 => write!(f, "SET [HL], Bit 7"),
            Bitset::ABit0 => write!(f, "SET A, Bit 0"),
            Bitset::ABit1 => write!(f, "SET A, Bit 1"),
            Bitset::ABit2 => write!(f, "SET A, Bit 2"),
            Bitset::ABit3 => write!(f, "SET A, Bit 3"),
            Bitset::ABit4 => write!(f, "SET A, Bit 4"),
            Bitset::ABit5 => write!(f, "SET A, Bit 5"),
            Bitset::ABit6 => write!(f, "SET A, Bit 6"),
            Bitset::ABit7 => write!(f, "SET A, Bit 7"),
        }
    }
}

#[cfg(test)]
mod test_set_bit {
    use super::Bitset;
    use crate::registers::{Bits16, Bits8, Bus};
    use crate::{Access, Cpu};
    use shared::execute;

    #[test]
    fn test_if_bit_6_in_register_b_is_set() {
        let src = 0b0000_1000;
        let expected = 0b0100_1000;
        let cpu = Cpu::default();
        let instruction = Bitset::BBit6;
        cpu.borrow_mut().registers.set(Bits8::B, src);

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let result = cpu.borrow_mut().registers.get(Bits8::B);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_bit_2_in_hl_is_reset() {
        let hl = 0xc008;
        let src = 0b0100_0000;
        let expected = 0b0100_0100;
        let cpu = Cpu::default();
        let instruction = Bitset::HLBit2;
        cpu.borrow_mut().registers.set(Bits16::HL, hl);
        cpu.memory().borrow_mut().set_u8(hl, src).unwrap();

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let result = cpu.memory().borrow_mut().get_u8(hl).unwrap();
        assert_eq!(result, expected);
    }
}
