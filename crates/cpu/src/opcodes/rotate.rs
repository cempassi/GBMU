use crate::{
    registers::{
        futures::{CbOperation, Set},
        Bits8, Rotation,
    },
    Registers,
};
use memory::Memory;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;

use super::decode::{Decode, Decoder};

/// RR r8
/// Rotate bits in register r8 right through carry.
/// RR [HL]
/// Rotate byte pointed to by HL right through carry.
/// RRA
/// Rotate register A right through carry.
///
/// C -> [7 -> 0] -> C

/// RRC r8
/// Rotate register r8 right.
/// RRC [HL]
/// Rotate byte pointed to by HL right.
/// RRCA
/// Rotate register A right.
///
/// C -> [7 -> 0] -> [7]

/// LR r8
/// Rotate bits in register r8 left through carry.
/// LR [HL]
/// Rotate byte pointed to by HL left through carry.
/// LRA
/// Rotate register A left through carry.
///
/// C <- [7 <- 0] <- C

/// LRC r8
/// Rotate register r8 left.
/// LRC [HL]
/// Rotate byte pointed to by HL left.
/// LRCA
/// Rotate register A left.
///
/// C <- [7 <- 0] <- [7]

/// Cycles: 8 | 16 for [HL]
///
/// Bytes: 2 | 4 for [HL]
///
/// Flags:
///
/// Z - Set if result is 0.
/// N - Unused
/// H - Unused
/// C - Set according to result.

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Rotate {
    LCB = 0x00,
    LCC = 0x01,
    LCD = 0x02,
    LCE = 0x03,
    LCH = 0x04,
    LCL = 0x05,
    LCHL = 0x06,
    LCA = 0x07,
    LB = 0x10,
    LC = 0x11,
    LD = 0x12,
    LE = 0x13,
    LH = 0x14,
    LL = 0x15,
    LHL = 0x16,
    LA = 0x17,
    RCB = 0x08,
    RCC = 0x09,
    RCD = 0x0A,
    RCE = 0x0B,
    RCH = 0x0C,
    RCL = 0x0D,
    RCHL = 0x0E,
    RCA = 0x0F,
    RB = 0x18,
    RC = 0x19,
    RD = 0x1A,
    RE = 0x1B,
    RH = 0x1C,
    RL = 0x1D,
    RHL = 0x1E,
    RA = 0x1F,
}

impl Decoder for Rotate {
    fn decode(self, registers: Registers, memory: Memory) -> Decode {
        Box::pin(self.exec(registers, memory))
    }
}

impl Rotate {
    pub async fn exec(self, registers: Registers, memory: Memory) -> Result<u8, Error> {
        let cycles = match self {
            Rotate::LA => registers.borrow_mut().left_carry(Bits8::A),
            Rotate::LB => registers.borrow_mut().left_carry(Bits8::B),
            Rotate::LC => registers.borrow_mut().left_carry(Bits8::C),
            Rotate::LD => registers.borrow_mut().left_carry(Bits8::D),
            Rotate::LE => registers.borrow_mut().left_carry(Bits8::E),
            Rotate::LH => registers.borrow_mut().left_carry(Bits8::H),
            Rotate::LL => registers.borrow_mut().left_carry(Bits8::L),
            Rotate::LCA => registers.borrow_mut().left_nocarry(Bits8::A),
            Rotate::LCB => registers.borrow_mut().left_nocarry(Bits8::B),
            Rotate::LCC => registers.borrow_mut().left_nocarry(Bits8::C),
            Rotate::LCD => registers.borrow_mut().left_nocarry(Bits8::D),
            Rotate::LCE => registers.borrow_mut().left_nocarry(Bits8::E),
            Rotate::LCH => registers.borrow_mut().left_nocarry(Bits8::H),
            Rotate::LCL => registers.borrow_mut().left_nocarry(Bits8::L),
            Rotate::RA => registers.borrow_mut().right_carry(Bits8::A),
            Rotate::RB => registers.borrow_mut().right_carry(Bits8::B),
            Rotate::RC => registers.borrow_mut().right_carry(Bits8::C),
            Rotate::RD => registers.borrow_mut().right_carry(Bits8::D),
            Rotate::RE => registers.borrow_mut().right_carry(Bits8::E),
            Rotate::RH => registers.borrow_mut().right_carry(Bits8::H),
            Rotate::RL => registers.borrow_mut().right_carry(Bits8::L),
            Rotate::RCA => registers.borrow_mut().right_nocarry(Bits8::A),
            Rotate::RCB => registers.borrow_mut().right_nocarry(Bits8::B),
            Rotate::RCC => registers.borrow_mut().right_nocarry(Bits8::C),
            Rotate::RCD => registers.borrow_mut().right_nocarry(Bits8::D),
            Rotate::RCE => registers.borrow_mut().right_nocarry(Bits8::E),
            Rotate::RCH => registers.borrow_mut().right_nocarry(Bits8::H),
            Rotate::RCL => registers.borrow_mut().right_nocarry(Bits8::L),
            Rotate::LHL => {
                Set::CbHL(CbOperation::RLCarry)
                    .run(registers, memory)
                    .await?
            }
            Rotate::LCHL => {
                Set::CbHL(CbOperation::RLNOCarry)
                    .run(registers, memory)
                    .await?
            }
            Rotate::RHL => {
                Set::CbHL(CbOperation::RRCarry)
                    .run(registers, memory)
                    .await?
            }
            Rotate::RCHL => {
                Set::CbHL(CbOperation::RRNoCarry)
                    .run(registers, memory)
                    .await?
            }
        };
        Ok(cycles)
    }
}

#[cfg(test)]
mod test_rotate {
    use super::Rotate;
    use crate::registers::{Bits16, Bits8, Bus, Flag};
    use crate::{executor, Registers};
    use memory::Memory;

    #[test]
    fn test_rotate_right_register_no_carry() {
        let src = 0b00010001;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Rotate::RCA;
        register.borrow_mut().set(Bits8::A, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits8::A);
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_rotate_right_register_with_carry() {
        let src = 0b00010000;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Rotate::RA;
        register.borrow_mut().set(Flag::C, true);
        register.borrow_mut().set(Bits8::A, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow_mut().get(Bits8::A);
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(!carry);
    }

    #[test]
    fn test_rotate_memory_right_hl_no_carry() {
        let hl = 0xc008;
        let src = 0b00010001;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Rotate::RCHL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get_u8(hl).unwrap();
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_rotate_right_hl_with_carry() {
        let hl = 0xc008;
        let src = 0b00010000;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Rotate::RHL;
        register.borrow_mut().set(Bits16::HL, hl);
        register.borrow_mut().set(Flag::C, true);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get_u8(hl).unwrap();
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(!carry);
    }

    #[test]
    fn test_rotate_register_left_no_carry() {
        let src = 0b10001000;
        let expected = 0b00010001;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Rotate::LCA;
        register.borrow_mut().set(Bits8::A, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits8::A);
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_rotate_register_left_with_carry() {
        let src = 0b10001000;
        let expected = 0b00010000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Rotate::LA;
        register.borrow_mut().set(Bits8::A, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow_mut().get(Bits8::A);
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_rotate_memory_hl_left_no_carry() {
        let hl = 0xc008;
        let src = 0b10001000;
        let expected = 0b00010001;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Rotate::LCHL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get_u8(hl).unwrap();
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_rotate_memory_hl_left_with_carry() {
        let hl = 0xc008;
        let src = 0b10001000;
        let expected = 0b00010000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Rotate::LHL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get_u8(hl).unwrap();
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }
}
