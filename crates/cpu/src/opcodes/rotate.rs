use crate::area::Bits8;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use super::cb_operation::CBOperation;

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

impl Rotate {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Rotate::LA => CBOperation::RotateLeftCarry.rotate(registers, Bits8::A),
            Rotate::LB => CBOperation::RotateLeftCarry.rotate(registers, Bits8::B),
            Rotate::LC => CBOperation::RotateLeftCarry.rotate(registers, Bits8::C),
            Rotate::LD => CBOperation::RotateLeftCarry.rotate(registers, Bits8::D),
            Rotate::LE => CBOperation::RotateLeftCarry.rotate(registers, Bits8::E),
            Rotate::LH => CBOperation::RotateLeftCarry.rotate(registers, Bits8::H),
            Rotate::LL => CBOperation::RotateLeftCarry.rotate(registers, Bits8::L),
            Rotate::LHL =>  CBOperation::RotateLeftCarry.rotate_hl(registers, memory).await,
            Rotate::LCA => CBOperation::RotateLeftNoCarry.rotate(registers, Bits8::A),
            Rotate::LCB => CBOperation::RotateLeftNoCarry.rotate(registers, Bits8::B),
            Rotate::LCC => CBOperation::RotateLeftNoCarry.rotate(registers, Bits8::C),
            Rotate::LCD => CBOperation::RotateLeftNoCarry.rotate(registers, Bits8::D),
            Rotate::LCE => CBOperation::RotateLeftNoCarry.rotate(registers, Bits8::E),
            Rotate::LCH => CBOperation::RotateLeftNoCarry.rotate(registers, Bits8::H),
            Rotate::LCL => CBOperation::RotateLeftNoCarry.rotate(registers, Bits8::L),
            Rotate::LCHL => CBOperation::RotateLeftNoCarry.rotate_hl(registers, memory).await,
            Rotate::RA => CBOperation::RotateRightCarry.rotate(registers, Bits8::A),
            Rotate::RB => CBOperation::RotateRightCarry.rotate(registers, Bits8::B),
            Rotate::RC => CBOperation::RotateRightCarry.rotate(registers, Bits8::C),
            Rotate::RD => CBOperation::RotateRightCarry.rotate(registers, Bits8::D),
            Rotate::RE => CBOperation::RotateRightCarry.rotate(registers, Bits8::E),
            Rotate::RH => CBOperation::RotateRightCarry.rotate(registers, Bits8::H),
            Rotate::RL => CBOperation::RotateRightCarry.rotate(registers, Bits8::L),
            Rotate::RHL => CBOperation::RotateRightCarry.rotate_hl(registers, memory).await,
            Rotate::RCA => CBOperation::RotateRightNoCarry.rotate(registers, Bits8::A),
            Rotate::RCB => CBOperation::RotateRightNoCarry.rotate(registers, Bits8::B),
            Rotate::RCC => CBOperation::RotateRightNoCarry.rotate(registers, Bits8::C),
            Rotate::RCD => CBOperation::RotateRightNoCarry.rotate(registers, Bits8::D),
            Rotate::RCE => CBOperation::RotateRightNoCarry.rotate(registers, Bits8::E),
            Rotate::RCH => CBOperation::RotateRightNoCarry.rotate(registers, Bits8::H),
            Rotate::RCL => CBOperation::RotateRightNoCarry.rotate(registers, Bits8::L),
            Rotate::RCHL => CBOperation::RotateRightNoCarry.rotate_hl(registers, memory).await,
        };
    }
}

#[cfg(test)]
mod test_rotate {
    use super::Rotate;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
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
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be true", carry);
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
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be false", carry);
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
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be true", carry);
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
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be false", carry);
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
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be true", carry);
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
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be true", carry);
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
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be true", carry);
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
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be true", carry);
        assert_eq!(result, expected);
        assert_eq!(carry, true);
    }
}
