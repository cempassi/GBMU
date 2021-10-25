use super::consts;
use crate::cpu::Registers;
use crate::registers::{futures::Set, Bits8, Bitwise};
use memory::Memory;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;

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

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
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
    fn decode(self, registers: Registers, memory: Memory) -> Decode {
        Box::pin(self.exec(registers, memory))
    }
}

impl Test {
    pub async fn exec(self, registers: Registers, memory: Memory) -> Result<u8, Error> {
        let cycles = match self {
            Test::BBit0 => registers.borrow_mut().test(Bits8::B, consts::BIT0),
            Test::BBit1 => registers.borrow_mut().test(Bits8::B, consts::BIT1),
            Test::BBit2 => registers.borrow_mut().test(Bits8::B, consts::BIT2),
            Test::BBit3 => registers.borrow_mut().test(Bits8::B, consts::BIT3),
            Test::BBit4 => registers.borrow_mut().test(Bits8::B, consts::BIT4),
            Test::BBit5 => registers.borrow_mut().test(Bits8::B, consts::BIT5),
            Test::BBit6 => registers.borrow_mut().test(Bits8::B, consts::BIT6),
            Test::BBit7 => registers.borrow_mut().test(Bits8::B, consts::BIT7),
            Test::CBit0 => registers.borrow_mut().test(Bits8::C, consts::BIT0),
            Test::CBit1 => registers.borrow_mut().test(Bits8::C, consts::BIT1),
            Test::CBit2 => registers.borrow_mut().test(Bits8::C, consts::BIT2),
            Test::CBit3 => registers.borrow_mut().test(Bits8::C, consts::BIT3),
            Test::CBit4 => registers.borrow_mut().test(Bits8::C, consts::BIT4),
            Test::CBit5 => registers.borrow_mut().test(Bits8::C, consts::BIT5),
            Test::CBit6 => registers.borrow_mut().test(Bits8::C, consts::BIT6),
            Test::CBit7 => registers.borrow_mut().test(Bits8::C, consts::BIT7),
            Test::DBit0 => registers.borrow_mut().test(Bits8::D, consts::BIT0),
            Test::DBit1 => registers.borrow_mut().test(Bits8::D, consts::BIT1),
            Test::DBit2 => registers.borrow_mut().test(Bits8::D, consts::BIT2),
            Test::DBit3 => registers.borrow_mut().test(Bits8::D, consts::BIT3),
            Test::DBit4 => registers.borrow_mut().test(Bits8::D, consts::BIT4),
            Test::DBit5 => registers.borrow_mut().test(Bits8::D, consts::BIT5),
            Test::DBit6 => registers.borrow_mut().test(Bits8::D, consts::BIT6),
            Test::DBit7 => registers.borrow_mut().test(Bits8::D, consts::BIT7),
            Test::EBit0 => registers.borrow_mut().test(Bits8::E, consts::BIT0),
            Test::EBit1 => registers.borrow_mut().test(Bits8::E, consts::BIT1),
            Test::EBit2 => registers.borrow_mut().test(Bits8::E, consts::BIT2),
            Test::EBit3 => registers.borrow_mut().test(Bits8::E, consts::BIT3),
            Test::EBit4 => registers.borrow_mut().test(Bits8::E, consts::BIT4),
            Test::EBit5 => registers.borrow_mut().test(Bits8::E, consts::BIT5),
            Test::EBit6 => registers.borrow_mut().test(Bits8::E, consts::BIT6),
            Test::EBit7 => registers.borrow_mut().test(Bits8::E, consts::BIT7),
            Test::HBit0 => registers.borrow_mut().test(Bits8::H, consts::BIT0),
            Test::HBit1 => registers.borrow_mut().test(Bits8::H, consts::BIT1),
            Test::HBit2 => registers.borrow_mut().test(Bits8::H, consts::BIT2),
            Test::HBit3 => registers.borrow_mut().test(Bits8::H, consts::BIT3),
            Test::HBit4 => registers.borrow_mut().test(Bits8::H, consts::BIT4),
            Test::HBit5 => registers.borrow_mut().test(Bits8::H, consts::BIT5),
            Test::HBit6 => registers.borrow_mut().test(Bits8::H, consts::BIT6),
            Test::HBit7 => registers.borrow_mut().test(Bits8::H, consts::BIT7),
            Test::LBit0 => registers.borrow_mut().test(Bits8::L, consts::BIT0),
            Test::LBit1 => registers.borrow_mut().test(Bits8::L, consts::BIT1),
            Test::LBit2 => registers.borrow_mut().test(Bits8::L, consts::BIT2),
            Test::LBit3 => registers.borrow_mut().test(Bits8::L, consts::BIT3),
            Test::LBit4 => registers.borrow_mut().test(Bits8::L, consts::BIT4),
            Test::LBit5 => registers.borrow_mut().test(Bits8::L, consts::BIT5),
            Test::LBit6 => registers.borrow_mut().test(Bits8::L, consts::BIT6),
            Test::LBit7 => registers.borrow_mut().test(Bits8::L, consts::BIT7),
            Test::ABit0 => registers.borrow_mut().test(Bits8::A, consts::BIT0),
            Test::ABit1 => registers.borrow_mut().test(Bits8::A, consts::BIT1),
            Test::ABit2 => registers.borrow_mut().test(Bits8::A, consts::BIT2),
            Test::ABit3 => registers.borrow_mut().test(Bits8::A, consts::BIT3),
            Test::ABit4 => registers.borrow_mut().test(Bits8::A, consts::BIT4),
            Test::ABit5 => registers.borrow_mut().test(Bits8::A, consts::BIT5),
            Test::ABit6 => registers.borrow_mut().test(Bits8::A, consts::BIT6),
            Test::ABit7 => registers.borrow_mut().test(Bits8::A, consts::BIT7),
            Test::HLBit0 => Set::TestHL(consts::BIT0).run(registers, memory).await?,
            Test::HLBit1 => Set::TestHL(consts::BIT1).run(registers, memory).await?,
            Test::HLBit2 => Set::TestHL(consts::BIT2).run(registers, memory).await?,
            Test::HLBit3 => Set::TestHL(consts::BIT3).run(registers, memory).await?,
            Test::HLBit4 => Set::TestHL(consts::BIT4).run(registers, memory).await?,
            Test::HLBit5 => Set::TestHL(consts::BIT5).run(registers, memory).await?,
            Test::HLBit6 => Set::TestHL(consts::BIT6).run(registers, memory).await?,
            Test::HLBit7 => Set::TestHL(consts::BIT7).run(registers, memory).await?,
        };
        Ok(cycles)
    }
}

#[cfg(test)]
mod test_test_bit {
    use super::Test;
    use crate::registers::{Bits16, Bits8, Bus, Flag};
    use crate::{executor, Registers};
    use memory::Memory;

    #[test]
    fn test_if_bit_6_in_register_b_is_not_zero() {
        let src = 0b0100_0000;
        let expected = false;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Test::BBit6;
        register.borrow_mut().set(Bits8::B, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let is_zero = register.borrow_mut().get(Flag::Z);
        assert!(is_zero == expected);
    }

    #[test]
    fn test_if_bit_4_in_register_d_is_zero() {
        let expected = true;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Test::DBit4;

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let is_zero = register.borrow_mut().get(Flag::Z);
        assert!(is_zero == expected);
    }

    #[test]
    fn test_if_bit_2_in_hl_is_not_zero() {
        let hl = 0xc008;
        let src = 0b0000_0100;
        let expected = false;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Test::HLBit2;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let is_zero = register.borrow_mut().get(Flag::Z);
        assert!(is_zero == expected);
    }
}
