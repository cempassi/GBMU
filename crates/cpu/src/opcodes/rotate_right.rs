use super::consts::{BIT0, CARRY, NO_CARRY};
use super::Data;
use crate::area::{Bits16, Bits8, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use memory::{Async, Memory};
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
pub enum RotateRight {
    CB = 0x08,
    CC = 0x09,
    CD = 0x0A,
    CE = 0x0B,
    CH = 0x0C,
    CL = 0x0D,
    CHL = 0x0E,
    CA = 0x0F,
    B = 0x18,
    C = 0x19,
    D = 0x1A,
    E = 0x1B,
    H = 0x1C,
    L = 0x1D,
    HL = 0x1E,
    A = 0x1F,
}

fn rotate(registers: Registers, area: Bits8, is_carried: bool) {
    let mut data = registers.borrow().get(area);
    let carry = (data & BIT0) != 0;
    data >>= 1;
    match is_carried {
        true => data |= (registers.borrow().get(Flag::C) as u8) << 7,
        false => data |= (carry as u8) << 7,
    };
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().set(Flag::C, carry);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    };
}

async fn rotate_hl(registers: Registers, memory: Memory, is_carried: bool) {
    let address = registers.borrow().get(Bits16::HL);
    let mut data = <Memory as Async<u8>>::get(memory.clone(), address)
        .await
        .unwrap();
    let carry = (data & BIT0) != 0;
    data >>= 1;
    match is_carried {
        true => data |= (registers.borrow().get(Flag::C) as u8) << 7,
        false => data |= (carry as u8) << 7,
    };
    <Memory as Async<u8>>::set(memory.clone(), address, data)
        .await
        .unwrap();
    registers.borrow_mut().set(Flag::C, carry);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    };
}

impl RotateRight {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            RotateRight::A => rotate(registers, Bits8::A, CARRY),
            RotateRight::B => rotate(registers, Bits8::B, CARRY),
            RotateRight::C => rotate(registers, Bits8::C, CARRY),
            RotateRight::D => rotate(registers, Bits8::D, CARRY),
            RotateRight::E => rotate(registers, Bits8::E, CARRY),
            RotateRight::H => rotate(registers, Bits8::H, CARRY),
            RotateRight::L => rotate(registers, Bits8::L, CARRY),
            RotateRight::HL => rotate_hl(registers, memory, CARRY).await,
            RotateRight::CA => rotate(registers, Bits8::A, NO_CARRY),
            RotateRight::CB => rotate(registers, Bits8::B, NO_CARRY),
            RotateRight::CC => rotate(registers, Bits8::C, NO_CARRY),
            RotateRight::CD => rotate(registers, Bits8::D, NO_CARRY),
            RotateRight::CE => rotate(registers, Bits8::E, NO_CARRY),
            RotateRight::CH => rotate(registers, Bits8::H, NO_CARRY),
            RotateRight::CL => rotate(registers, Bits8::L, NO_CARRY),
            RotateRight::CHL => rotate_hl(registers, memory, NO_CARRY).await,
        };
    }
}

#[cfg(test)]
mod test_rotate_right {
    use super::RotateRight;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_rotate_register_right_no_carry() {
        let src = 0b00010001;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateRight::CA;
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
    fn test_rotate_register_right_with_carry() {
        let src = 0b00010000;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateRight::A;
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
    fn test_rotate_memory_hl_right_no_carry() {
        let hl = 0xc008;
        let src = 0b00010001;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateRight::CHL;
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
    fn test_rotate_memory_hl_right_with_carry() {
        let hl = 0xc008;
        let src = 0b00010000;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateRight::HL;
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
}
