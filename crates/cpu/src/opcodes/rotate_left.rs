use super::consts::BIT7;
use crate::area::{Bits16, Bits8, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// RL r8
/// Rotate bits in register r8 left through carry.
/// RL [HL]
/// Rotate byte pointed to by HL left through carry.
/// RLA
/// Rotate register A left through carry.
///
/// C <- [7 <- 0] <- C

/// RLC r8
/// Rotate register r8 left.
/// RLC [HL]
/// Rotate byte pointed to by HL left.
/// RLCA
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
pub enum RotateLeft {
    B = 0x00,
    C = 0x01,
    D = 0x02,
    E = 0x03,
    H = 0x04,
    L = 0x05,
    HL = 0x06,
    A = 0x07,
    CB = 0x10,
    CC = 0x11,
    CD = 0x12,
    CE = 0x13,
    CH = 0x14,
    CL = 0x15,
    CHL = 0x16,
    CA = 0x17,
}

fn rotate_carry(registers: Registers, area: Bits8) {
    let mut data = registers.borrow().get(area);
    let carry = (data & BIT7) != 0;
    data <<= 1;
    data |= registers.borrow().get(Flag::C) as u8;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().set(Flag::C, carry);
}

fn rotate(registers: Registers, area: Bits8) {
    let mut data = registers.borrow().get(area);
    let holder = (data & BIT7) != 0;
    data <<= 1;
    data |= holder as u8;
    registers.borrow_mut().set(area, data);
}

impl RotateLeft {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            RotateLeft::A => rotate_carry(registers, Bits8::A),
            RotateLeft::B => rotate_carry(registers, Bits8::B),
            RotateLeft::C => rotate_carry(registers, Bits8::C),
            RotateLeft::D => rotate_carry(registers, Bits8::D),
            RotateLeft::E => rotate_carry(registers, Bits8::E),
            RotateLeft::H => rotate_carry(registers, Bits8::H),
            RotateLeft::L => rotate_carry(registers, Bits8::L),
            RotateLeft::CA => rotate(registers, Bits8::A),
            RotateLeft::CB => rotate(registers, Bits8::B),
            RotateLeft::CC => rotate(registers, Bits8::C),
            RotateLeft::CD => rotate(registers, Bits8::D),
            RotateLeft::CE => rotate(registers, Bits8::E),
            RotateLeft::CH => rotate(registers, Bits8::H),
            RotateLeft::CL => rotate(registers, Bits8::L),
            RotateLeft::HL => {
                let dst = registers.borrow().get(Bits16::HL);
                let mut data = <Memory as Async>::get(memory.clone(), dst).await.unwrap();
                let carry = (data & BIT7) != 0;
                data <<= 1;
                data |= registers.borrow().get(Flag::C) as u8;
                registers.borrow_mut().set(Flag::C, carry);
                <Memory as Async>::set(memory, dst, data).await.unwrap()
            }
            RotateLeft::CHL => {
                let dst = registers.borrow().get(Bits16::HL);
                let mut data = <Memory as Async>::get(memory.clone(), dst).await.unwrap();
                let holder = (data & BIT7) != 0;
                data <<= 1;
                data |= holder as u8;
                <Memory as Async>::set(memory, dst, data).await.unwrap()
            }
        };
    }
}

#[cfg(test)]
mod test_rotate_left {
    use super::RotateLeft;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_rotate_register_left_no_carry() {
        let src = 0b10001000;
        let expected = 0b00010001;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateLeft::CA;
        register.borrow_mut().set(Bits8::A, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits8::A);
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rotate_register_left_with_carry() {
        let src = 0b10001000;
        let expected = 0b00010000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateLeft::A;
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
        let instruction = RotateLeft::CHL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get(hl).unwrap();
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rotate_memory_hl_left_with_carry() {
        let hl = 0xc008;
        let src = 0b10001000;
        let expected = 0b00010000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateLeft::HL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get(hl).unwrap();
        let carry = register.borrow_mut().get(Flag::C);
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be true", carry);
        assert_eq!(result, expected);
        assert_eq!(carry, true);
    }
}
