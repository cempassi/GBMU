use super::consts::BIT7;
use crate::area::{Bits16, Bits8, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// SLA r8
/// Shift Left Arithmetic register r8.
///
/// C <- [7 <- 0] <- 0
///
/// Cycles: 2
///
/// Bytes: 2
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
pub enum ShiftLeft {
    B = 0x20,
    C = 0x21,
    D = 0x22,
    E = 0x23,
    H = 0x24,
    L = 0x25,
    HL = 0x26,
    A = 0x27,
}

fn shift(registers: Registers, area: Bits8) {
    let mut data = registers.borrow().get(area);
    let carry = (data & BIT7) != 0;
    data <<= 1;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().set(Flag::C, carry);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    };
}

async fn shift_hl(registers: Registers, memory: Memory) {
    let address = registers.borrow().get(Bits16::HL);
    let mut data = <Memory as Async>::get(memory.clone(), address)
        .await
        .unwrap();
    let carry = (data & BIT7) != 0;
    data <<= 1;
    <Memory as Async>::set(memory.clone(), address, data)
        .await
        .unwrap();
    registers.borrow_mut().set(Flag::C, carry);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    };
}

impl ShiftLeft {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            ShiftLeft::B => shift(registers, Bits8::B),
            ShiftLeft::C => shift(registers, Bits8::C),
            ShiftLeft::D => shift(registers, Bits8::D),
            ShiftLeft::E => shift(registers, Bits8::E),
            ShiftLeft::H => shift(registers, Bits8::H),
            ShiftLeft::L => shift(registers, Bits8::H),
            ShiftLeft::A => shift(registers, Bits8::A),
            ShiftLeft::HL => shift_hl(registers, memory).await,
        }
    }
}

#[cfg(test)]
mod test_shift_left {
    use super::ShiftLeft;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_shift_register_left() {
        let src = 0b10001000;
        let expected = 0b00010000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = ShiftLeft::A;
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
    fn test_shift_memory_hl_left() {
        let hl = 0xc008;
        let src = 0b10001000;
        let expected = 0b00010000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = ShiftLeft::HL;
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
