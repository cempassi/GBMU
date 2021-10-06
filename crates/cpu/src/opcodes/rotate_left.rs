use super::consts::BIT7;
use super::Data;
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
    CB = 0x00,
    CC = 0x01,
    CD = 0x02,
    CE = 0x03,
    CH = 0x04,
    CL = 0x05,
    CHL = 0x06,
    CA = 0x07,
    B = 0x10,
    C = 0x11,
    D = 0x12,
    E = 0x13,
    H = 0x14,
    L = 0x15,
    HL = 0x16,
    A = 0x17,
}

fn rotate(registers: Registers, dst: Data<Bits8>) {
    let area = match dst {
        Data::Carry(area) => area,
        Data::NoCarry(area) => area,
    };
    let mut data = registers.borrow().get(area);
    let carry = (data & BIT7) != 0;
    data <<= 1;
    match dst {
        Data::Carry(_) => data |= registers.borrow().get(Flag::C) as u8,
        Data::NoCarry(_) => data |= carry as u8,
    };
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().set(Flag::C, carry);
}

async fn rotate_hl(registers: Registers, dst: Data<Memory>) {
    let memory = match dst {
        Data::Carry(ref memory) => memory,
        Data::NoCarry(ref memory) => memory,
    };
    let address = registers.borrow().get(Bits16::HL);
    let mut data = <Memory as Async>::get(memory.clone(), address)
        .await
        .unwrap();
    let carry = (data & BIT7) != 0;
    data <<= 1;
    match dst {
        Data::Carry(_) => data |= registers.borrow().get(Flag::C) as u8,
        Data::NoCarry(_) => data |= carry as u8,
    };
    <Memory as Async>::set(memory.clone(), address, data)
        .await
        .unwrap();
    registers.borrow_mut().set(Flag::C, carry);
}

impl RotateLeft {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            RotateLeft::A => rotate(registers, Data::Carry(Bits8::A)),
            RotateLeft::B => rotate(registers, Data::Carry(Bits8::B)),
            RotateLeft::C => rotate(registers, Data::Carry(Bits8::C)),
            RotateLeft::D => rotate(registers, Data::Carry(Bits8::D)),
            RotateLeft::E => rotate(registers, Data::Carry(Bits8::E)),
            RotateLeft::H => rotate(registers, Data::Carry(Bits8::H)),
            RotateLeft::L => rotate(registers, Data::Carry(Bits8::L)),
            RotateLeft::HL => rotate_hl(registers, Data::Carry(memory)).await,
            RotateLeft::CA => rotate(registers, Data::NoCarry(Bits8::A)),
            RotateLeft::CB => rotate(registers, Data::NoCarry(Bits8::B)),
            RotateLeft::CC => rotate(registers, Data::NoCarry(Bits8::C)),
            RotateLeft::CD => rotate(registers, Data::NoCarry(Bits8::D)),
            RotateLeft::CE => rotate(registers, Data::NoCarry(Bits8::E)),
            RotateLeft::CH => rotate(registers, Data::NoCarry(Bits8::H)),
            RotateLeft::CL => rotate(registers, Data::NoCarry(Bits8::L)),
            RotateLeft::CHL => rotate_hl(registers, Data::NoCarry(memory)).await,
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
