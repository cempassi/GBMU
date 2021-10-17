use super::consts::{BIT0, BIT7};
use crate::opcodes::Src;
use crate::{
    registers::{Bits8, Bus, Flag},
    Registers,
};
use memory::Memory;
use num_enum::TryFromPrimitive;

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

/// Cycles: 2 | 4 for [HL]
///
/// Bytes: 2 | 4 for [HL]
///
/// Flags:
///
/// Z - Set if result is 0.
/// N - Unused
/// H - Unused
/// C - Set according to result.

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
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

pub enum Rotation {
    Left,
    LeftNoCarry,
    Right,
    RightNoCarry,
}

impl Rotation {
    fn rotation(self, registers: &Registers, data: u8) -> u8 {
        let carry = match self {
            Rotation::Left | Rotation::LeftNoCarry => (data & BIT7) != 0,
            Rotation::Right | Rotation::RightNoCarry => (data & BIT0) != 0,
        };
        let data = match self {
            Rotation::Left => (data << 1) | registers.borrow().get(Flag::C) as u8,
            Rotation::LeftNoCarry => (data << 1) | carry as u8,
            Rotation::Right => (data >> 1) | ((registers.borrow().get(Flag::C) as u8) << 7),
            Rotation::RightNoCarry => (data >> 1) | ((carry as u8) << 7),
        };
        registers.borrow_mut().set(Flag::C, carry);
        registers.borrow_mut().set(Flag::Z, data == 0);
        data
    }

    pub(crate) async fn rotate(self, src: Src, registers: Registers, memory: Memory) {
        let data = src.get(registers.clone(), memory.clone()).await;
        let data = self.rotation(&registers, data);
        src.set(registers, memory, data).await;
    }
}

impl Rotate {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Rotate::LA => Rotation::Left.rotate(Src::Register(Bits8::A), registers, memory),
            Rotate::LB => Rotation::Left.rotate(Src::Register(Bits8::B), registers, memory),
            Rotate::LC => Rotation::Left.rotate(Src::Register(Bits8::C), registers, memory),
            Rotate::LD => Rotation::Left.rotate(Src::Register(Bits8::D), registers, memory),
            Rotate::LE => Rotation::Left.rotate(Src::Register(Bits8::E), registers, memory),
            Rotate::LH => Rotation::Left.rotate(Src::Register(Bits8::H), registers, memory),
            Rotate::LL => Rotation::Left.rotate(Src::Register(Bits8::L), registers, memory),
            Rotate::LHL => Rotation::Left.rotate(Src::Pointer, registers, memory),
            Rotate::LCA => Rotation::LeftNoCarry.rotate(Src::Register(Bits8::A), registers, memory),
            Rotate::LCB => Rotation::LeftNoCarry.rotate(Src::Register(Bits8::B), registers, memory),
            Rotate::LCC => Rotation::LeftNoCarry.rotate(Src::Register(Bits8::C), registers, memory),
            Rotate::LCD => Rotation::LeftNoCarry.rotate(Src::Register(Bits8::D), registers, memory),
            Rotate::LCE => Rotation::LeftNoCarry.rotate(Src::Register(Bits8::E), registers, memory),
            Rotate::LCH => Rotation::LeftNoCarry.rotate(Src::Register(Bits8::H), registers, memory),
            Rotate::LCL => Rotation::LeftNoCarry.rotate(Src::Register(Bits8::L), registers, memory),
            Rotate::LCHL => Rotation::LeftNoCarry.rotate(Src::Pointer, registers, memory),
            Rotate::RA => Rotation::Right.rotate(Src::Register(Bits8::A), registers, memory),
            Rotate::RB => Rotation::Right.rotate(Src::Register(Bits8::B), registers, memory),
            Rotate::RC => Rotation::Right.rotate(Src::Register(Bits8::C), registers, memory),
            Rotate::RD => Rotation::Right.rotate(Src::Register(Bits8::D), registers, memory),
            Rotate::RE => Rotation::Right.rotate(Src::Register(Bits8::E), registers, memory),
            Rotate::RH => Rotation::Right.rotate(Src::Register(Bits8::H), registers, memory),
            Rotate::RL => Rotation::Right.rotate(Src::Register(Bits8::L), registers, memory),
            Rotate::RHL => Rotation::Right.rotate(Src::Pointer, registers, memory),
            Rotate::RCA => {
                Rotation::RightNoCarry.rotate(Src::Register(Bits8::A), registers, memory)
            }
            Rotate::RCB => {
                Rotation::RightNoCarry.rotate(Src::Register(Bits8::B), registers, memory)
            }
            Rotate::RCC => {
                Rotation::RightNoCarry.rotate(Src::Register(Bits8::C), registers, memory)
            }
            Rotate::RCD => {
                Rotation::RightNoCarry.rotate(Src::Register(Bits8::D), registers, memory)
            }
            Rotate::RCE => {
                Rotation::RightNoCarry.rotate(Src::Register(Bits8::E), registers, memory)
            }
            Rotate::RCH => {
                Rotation::RightNoCarry.rotate(Src::Register(Bits8::H), registers, memory)
            }
            Rotate::RCL => {
                Rotation::RightNoCarry.rotate(Src::Register(Bits8::L), registers, memory)
            }
            Rotate::RCHL => Rotation::RightNoCarry.rotate(Src::Pointer, registers, memory),
        }
        .await;
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
        assert_eq!(carry, true);
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
        assert_eq!(carry, false);
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
        assert_eq!(carry, true);
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
        assert_eq!(carry, false);
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
        assert_eq!(carry, true);
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
        assert_eq!(carry, true);
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
        assert_eq!(carry, true);
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
        assert_eq!(carry, true);
    }
}
