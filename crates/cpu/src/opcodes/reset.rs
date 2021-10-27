use super::consts;
use crate::cpu::Registers;
use crate::registers::{
    futures::{CbOperation as Operation, Set},
    Bits8, Bitwise,
};
use memory::Memory;
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
pub enum Reset {
    BBit0 = 0x80,
    BBit1 = 0x88,
    BBit2 = 0x90,
    BBit3 = 0x98,
    BBit4 = 0xA0,
    BBit5 = 0xA8,
    BBit6 = 0xB0,
    BBit7 = 0xB8,
    CBit0 = 0x81,
    CBit1 = 0x89,
    CBit2 = 0x91,
    CBit3 = 0x99,
    CBit4 = 0xA1,
    CBit5 = 0xA9,
    CBit6 = 0xB1,
    CBit7 = 0xB9,
    DBit0 = 0x82,
    DBit1 = 0x8A,
    DBit2 = 0x92,
    DBit3 = 0x9A,
    DBit4 = 0xA2,
    DBit5 = 0xAA,
    DBit6 = 0xB2,
    DBit7 = 0xBA,
    EBit0 = 0x83,
    EBit1 = 0x8B,
    EBit2 = 0x93,
    EBit3 = 0x9B,
    EBit4 = 0xA3,
    EBit5 = 0xAB,
    EBit6 = 0xB3,
    EBit7 = 0xBB,
    HBit0 = 0x84,
    HBit1 = 0x8C,
    HBit2 = 0x94,
    HBit3 = 0x9C,
    HBit4 = 0xA4,
    HBit5 = 0xAC,
    HBit6 = 0xB4,
    HBit7 = 0xBC,
    LBit0 = 0x85,
    LBit1 = 0x8D,
    LBit2 = 0x95,
    LBit3 = 0x9D,
    LBit4 = 0xA5,
    LBit5 = 0xAD,
    LBit6 = 0xB5,
    LBit7 = 0xBD,
    HLBit0 = 0x86,
    HLBit1 = 0x8E,
    HLBit2 = 0x96,
    HLBit3 = 0x9E,
    HLBit4 = 0xA6,
    HLBit5 = 0xAE,
    HLBit6 = 0xB6,
    HLBit7 = 0xBE,
    ABit0 = 0x87,
    ABit1 = 0x8F,
    ABit2 = 0x97,
    ABit3 = 0x9F,
    ABit4 = 0xA7,
    ABit5 = 0xAF,
    ABit6 = 0xB7,
    ABit7 = 0xBF,
}

impl Decoder for Reset {
    fn decode(self, registers: Registers, memory: Memory) -> Decode {
        Box::pin(self.exec(registers, memory))
    }
}

impl Reset {
    pub async fn exec(self, registers: Registers, memory: Memory) -> Result<u8, Error> {
        let cycles = match self {
            Reset::BBit0 => registers.borrow_mut().reset(Bits8::B, consts::BIT0),
            Reset::BBit1 => registers.borrow_mut().reset(Bits8::B, consts::BIT1),
            Reset::BBit2 => registers.borrow_mut().reset(Bits8::B, consts::BIT2),
            Reset::BBit3 => registers.borrow_mut().reset(Bits8::B, consts::BIT3),
            Reset::BBit4 => registers.borrow_mut().reset(Bits8::B, consts::BIT4),
            Reset::BBit5 => registers.borrow_mut().reset(Bits8::B, consts::BIT5),
            Reset::BBit6 => registers.borrow_mut().reset(Bits8::B, consts::BIT6),
            Reset::BBit7 => registers.borrow_mut().reset(Bits8::B, consts::BIT7),
            Reset::CBit0 => registers.borrow_mut().reset(Bits8::C, consts::BIT0),
            Reset::CBit1 => registers.borrow_mut().reset(Bits8::C, consts::BIT1),
            Reset::CBit2 => registers.borrow_mut().reset(Bits8::C, consts::BIT2),
            Reset::CBit3 => registers.borrow_mut().reset(Bits8::C, consts::BIT3),
            Reset::CBit4 => registers.borrow_mut().reset(Bits8::C, consts::BIT4),
            Reset::CBit5 => registers.borrow_mut().reset(Bits8::C, consts::BIT5),
            Reset::CBit6 => registers.borrow_mut().reset(Bits8::C, consts::BIT6),
            Reset::CBit7 => registers.borrow_mut().reset(Bits8::C, consts::BIT7),
            Reset::DBit0 => registers.borrow_mut().reset(Bits8::D, consts::BIT0),
            Reset::DBit1 => registers.borrow_mut().reset(Bits8::D, consts::BIT1),
            Reset::DBit2 => registers.borrow_mut().reset(Bits8::D, consts::BIT2),
            Reset::DBit3 => registers.borrow_mut().reset(Bits8::D, consts::BIT3),
            Reset::DBit4 => registers.borrow_mut().reset(Bits8::D, consts::BIT4),
            Reset::DBit5 => registers.borrow_mut().reset(Bits8::D, consts::BIT5),
            Reset::DBit6 => registers.borrow_mut().reset(Bits8::D, consts::BIT6),
            Reset::DBit7 => registers.borrow_mut().reset(Bits8::D, consts::BIT7),
            Reset::EBit0 => registers.borrow_mut().reset(Bits8::E, consts::BIT0),
            Reset::EBit1 => registers.borrow_mut().reset(Bits8::E, consts::BIT1),
            Reset::EBit2 => registers.borrow_mut().reset(Bits8::E, consts::BIT2),
            Reset::EBit3 => registers.borrow_mut().reset(Bits8::E, consts::BIT3),
            Reset::EBit4 => registers.borrow_mut().reset(Bits8::E, consts::BIT4),
            Reset::EBit5 => registers.borrow_mut().reset(Bits8::E, consts::BIT5),
            Reset::EBit6 => registers.borrow_mut().reset(Bits8::E, consts::BIT6),
            Reset::EBit7 => registers.borrow_mut().reset(Bits8::E, consts::BIT7),
            Reset::HBit0 => registers.borrow_mut().reset(Bits8::H, consts::BIT0),
            Reset::HBit1 => registers.borrow_mut().reset(Bits8::H, consts::BIT1),
            Reset::HBit2 => registers.borrow_mut().reset(Bits8::H, consts::BIT2),
            Reset::HBit3 => registers.borrow_mut().reset(Bits8::H, consts::BIT3),
            Reset::HBit4 => registers.borrow_mut().reset(Bits8::H, consts::BIT4),
            Reset::HBit5 => registers.borrow_mut().reset(Bits8::H, consts::BIT5),
            Reset::HBit6 => registers.borrow_mut().reset(Bits8::H, consts::BIT6),
            Reset::HBit7 => registers.borrow_mut().reset(Bits8::H, consts::BIT7),
            Reset::LBit0 => registers.borrow_mut().reset(Bits8::L, consts::BIT0),
            Reset::LBit1 => registers.borrow_mut().reset(Bits8::L, consts::BIT1),
            Reset::LBit2 => registers.borrow_mut().reset(Bits8::L, consts::BIT2),
            Reset::LBit3 => registers.borrow_mut().reset(Bits8::L, consts::BIT3),
            Reset::LBit4 => registers.borrow_mut().reset(Bits8::L, consts::BIT4),
            Reset::LBit5 => registers.borrow_mut().reset(Bits8::L, consts::BIT5),
            Reset::LBit6 => registers.borrow_mut().reset(Bits8::L, consts::BIT6),
            Reset::LBit7 => registers.borrow_mut().reset(Bits8::L, consts::BIT7),
            Reset::ABit0 => registers.borrow_mut().reset(Bits8::A, consts::BIT0),
            Reset::ABit1 => registers.borrow_mut().reset(Bits8::A, consts::BIT1),
            Reset::ABit2 => registers.borrow_mut().reset(Bits8::A, consts::BIT2),
            Reset::ABit3 => registers.borrow_mut().reset(Bits8::A, consts::BIT3),
            Reset::ABit4 => registers.borrow_mut().reset(Bits8::A, consts::BIT4),
            Reset::ABit5 => registers.borrow_mut().reset(Bits8::A, consts::BIT5),
            Reset::ABit6 => registers.borrow_mut().reset(Bits8::A, consts::BIT6),
            Reset::ABit7 => registers.borrow_mut().reset(Bits8::A, consts::BIT7),
            Reset::HLBit0 => {
                Set::CbHL(Operation::Reset(consts::BIT0))
                    .run(registers, memory)
                    .await?
            }
            Reset::HLBit1 => {
                Set::CbHL(Operation::Reset(consts::BIT1))
                    .run(registers, memory)
                    .await?
            }
            Reset::HLBit2 => {
                Set::CbHL(Operation::Reset(consts::BIT2))
                    .run(registers, memory)
                    .await?
            }
            Reset::HLBit3 => {
                Set::CbHL(Operation::Reset(consts::BIT3))
                    .run(registers, memory)
                    .await?
            }
            Reset::HLBit4 => {
                Set::CbHL(Operation::Reset(consts::BIT4))
                    .run(registers, memory)
                    .await?
            }
            Reset::HLBit5 => {
                Set::CbHL(Operation::Reset(consts::BIT5))
                    .run(registers, memory)
                    .await?
            }
            Reset::HLBit6 => {
                Set::CbHL(Operation::Reset(consts::BIT6))
                    .run(registers, memory)
                    .await?
            }
            Reset::HLBit7 => {
                Set::CbHL(Operation::Reset(consts::BIT7))
                    .run(registers, memory)
                    .await?
            }
        };
        Ok(cycles)
    }
}

impl fmt::Display for Reset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Reset::BBit0 => write!(f, "RESET B, Bit 0"),
            Reset::BBit1 => write!(f, "RESET B, Bit 1"),
            Reset::BBit2 => write!(f, "RESET B, Bit 2"),
            Reset::BBit3 => write!(f, "RESET B, Bit 3"),
            Reset::BBit4 => write!(f, "RESET B, Bit 4"),
            Reset::BBit5 => write!(f, "RESET B, Bit 5"),
            Reset::BBit6 => write!(f, "RESET B, Bit 6"),
            Reset::BBit7 => write!(f, "RESET B, Bit 7"),
            Reset::CBit0 => write!(f, "RESET C, Bit 0"),
            Reset::CBit1 => write!(f, "RESET C, Bit 1"),
            Reset::CBit2 => write!(f, "RESET C, Bit 2"),
            Reset::CBit3 => write!(f, "RESET C, Bit 3"),
            Reset::CBit4 => write!(f, "RESET C, Bit 4"),
            Reset::CBit5 => write!(f, "RESET C, Bit 5"),
            Reset::CBit6 => write!(f, "RESET C, Bit 6"),
            Reset::CBit7 => write!(f, "RESET C, Bit 7"),
            Reset::DBit0 => write!(f, "RESET D, Bit 0"),
            Reset::DBit1 => write!(f, "RESET D, Bit 1"),
            Reset::DBit2 => write!(f, "RESET D, Bit 2"),
            Reset::DBit3 => write!(f, "RESET D, Bit 3"),
            Reset::DBit4 => write!(f, "RESET D, Bit 4"),
            Reset::DBit5 => write!(f, "RESET D, Bit 5"),
            Reset::DBit6 => write!(f, "RESET D, Bit 6"),
            Reset::DBit7 => write!(f, "RESET D, Bit 7"),
            Reset::EBit0 => write!(f, "RESET E, Bit 0"),
            Reset::EBit1 => write!(f, "RESET E, Bit 1"),
            Reset::EBit2 => write!(f, "RESET E, Bit 2"),
            Reset::EBit3 => write!(f, "RESET E, Bit 3"),
            Reset::EBit4 => write!(f, "RESET E, Bit 4"),
            Reset::EBit5 => write!(f, "RESET E, Bit 5"),
            Reset::EBit6 => write!(f, "RESET E, Bit 6"),
            Reset::EBit7 => write!(f, "RESET E, Bit 7"),
            Reset::HBit0 => write!(f, "RESET H, Bit 0"),
            Reset::HBit1 => write!(f, "RESET H, Bit 1"),
            Reset::HBit2 => write!(f, "RESET H, Bit 2"),
            Reset::HBit3 => write!(f, "RESET H, Bit 3"),
            Reset::HBit4 => write!(f, "RESET H, Bit 4"),
            Reset::HBit5 => write!(f, "RESET H, Bit 5"),
            Reset::HBit6 => write!(f, "RESET H, Bit 6"),
            Reset::HBit7 => write!(f, "RESET H, Bit 7"),
            Reset::LBit0 => write!(f, "RESET L, Bit 0"),
            Reset::LBit1 => write!(f, "RESET L, Bit 1"),
            Reset::LBit2 => write!(f, "RESET L, Bit 2"),
            Reset::LBit3 => write!(f, "RESET L, Bit 3"),
            Reset::LBit4 => write!(f, "RESET L, Bit 4"),
            Reset::LBit5 => write!(f, "RESET L, Bit 5"),
            Reset::LBit6 => write!(f, "RESET L, Bit 6"),
            Reset::LBit7 => write!(f, "RESET L, Bit 7"),
            Reset::HLBit0 => write!(f, "RESET [HL], Bit 0"),
            Reset::HLBit1 => write!(f, "RESET [HL], Bit 1"),
            Reset::HLBit2 => write!(f, "RESET [HL], Bit 2"),
            Reset::HLBit3 => write!(f, "RESET [HL], Bit 3"),
            Reset::HLBit4 => write!(f, "RESET [HL], Bit 4"),
            Reset::HLBit5 => write!(f, "RESET [HL], Bit 5"),
            Reset::HLBit6 => write!(f, "RESET [HL], Bit 6"),
            Reset::HLBit7 => write!(f, "RESET [HL], Bit 7"),
            Reset::ABit0 => write!(f, "RESET A, Bit 0"),
            Reset::ABit1 => write!(f, "RESET A, Bit 1"),
            Reset::ABit2 => write!(f, "RESET A, Bit 2"),
            Reset::ABit3 => write!(f, "RESET A, Bit 3"),
            Reset::ABit4 => write!(f, "RESET A, Bit 4"),
            Reset::ABit5 => write!(f, "RESET A, Bit 5"),
            Reset::ABit6 => write!(f, "RESET A, Bit 6"),
            Reset::ABit7 => write!(f, "RESET A, Bit 7"),
        }
    }
}

#[cfg(test)]
mod test_reset_bit {
    use super::Reset;
    use crate::registers::{Bits16, Bits8, Bus};
    use crate::{executor, Registers};
    use memory::Memory;

    #[test]
    fn test_if_bit_6_in_register_b_is_reset() {
        let src = 0b0100_1000;
        let expected = 0b0000_1000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Reset::BBit6;
        register.borrow_mut().set(Bits8::B, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow_mut().get(Bits8::B);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_bit_2_in_hl_is_reset() {
        let hl = 0xc008;
        let src = 0b0100_0100;
        let expected = 0b0100_0000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Reset::HLBit2;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register, memory.clone())));

        let result = memory.borrow_mut().get_u8(hl).unwrap();
        assert_eq!(result, expected);
    }
}
