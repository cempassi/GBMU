use super::consts::BIT0;
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

fn rotate(registers: Registers, dst: Data<Bits8>) {
    let area = match dst {
        Data::Carry(area) => area,
        Data::NoCarry(area) => area,
    };
    let mut data = registers.borrow().get(area);
    let carry = (data & BIT0) != 0;
    data >>= 1;
    match dst {
        Data::Carry(_) => data |= (registers.borrow().get(Flag::C) as u8) << 7,
        Data::NoCarry(_) => data |= (carry as u8) << 7,
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
    let carry = (data & BIT0) != 0;
    data >>= 1;
    match dst {
        Data::Carry(_) => data |= (registers.borrow().get(Flag::C) as u8) << 7,
        Data::NoCarry(_) => data |= (carry as u8) << 7,
    };
    <Memory as Async>::set(memory.clone(), address, data)
        .await
        .unwrap();
    registers.borrow_mut().set(Flag::C, carry);
}

impl RotateRight {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            RotateRight::A => rotate(registers, Data::Carry(Bits8::A)),
            RotateRight::B => rotate(registers, Data::Carry(Bits8::B)),
            RotateRight::C => rotate(registers, Data::Carry(Bits8::C)),
            RotateRight::D => rotate(registers, Data::Carry(Bits8::D)),
            RotateRight::E => rotate(registers, Data::Carry(Bits8::E)),
            RotateRight::H => rotate(registers, Data::Carry(Bits8::H)),
            RotateRight::L => rotate(registers, Data::Carry(Bits8::L)),
            RotateRight::HL => rotate_hl(registers, Data::Carry(memory)).await,
            RotateRight::CA => rotate(registers, Data::NoCarry(Bits8::A)),
            RotateRight::CB => rotate(registers, Data::NoCarry(Bits8::B)),
            RotateRight::CC => rotate(registers, Data::NoCarry(Bits8::C)),
            RotateRight::CD => rotate(registers, Data::NoCarry(Bits8::D)),
            RotateRight::CE => rotate(registers, Data::NoCarry(Bits8::E)),
            RotateRight::CH => rotate(registers, Data::NoCarry(Bits8::H)),
            RotateRight::CL => rotate(registers, Data::NoCarry(Bits8::L)),
            RotateRight::CHL => rotate_hl(registers, Data::NoCarry(memory)).await,
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
    fn test_rotate_memory_hl_right_with_carry() {
        let hl = 0xc008;
        let src = 0b00010000;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateRight::HL;
        register.borrow_mut().set(Bits16::HL, hl);
        register.borrow_mut().set(Flag::C, true);
        memory.borrow_mut().set(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get(hl).unwrap();
        let carry = register.borrow_mut().get(Flag::C);
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be false", carry);
        assert_eq!(result, expected);
        assert_eq!(carry, false);
    }
}
