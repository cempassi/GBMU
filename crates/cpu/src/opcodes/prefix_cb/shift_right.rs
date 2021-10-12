use super::consts::BIT0;
use crate::area::{Bits16, Bits8, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::opcodes::prefix_cb::consts::BIT7;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// 10. SRA n
/// Description:
///  Shift n right into Carry. MSB doesn't change.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// SRA         A          0xCB2F 8
/// SRA         B          0xCB28 8
/// SRA         C          0xCB29 8
/// SRA         D          0xCB2A 8
/// SRA         E          0xCB2B 8
/// SRA         H          0xCB2C 8
/// SRA         L          0xCB2D 8
/// SRA         (HL)       0xCB2E 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum ShiftRight {
    A = 0x2f,
    B = 0x28,
    C = 0x29,
    D = 0x2a,
    E = 0x2b,
    H = 0x2c,
    L = 0x2d,
    HL = 0x2e,
}

fn shift_right(registers: Registers, area: Bits8) {
    let mut data = registers.borrow().get(area);
    let bit7 = data & BIT7;
    let carry = (data & BIT0) != 0;
    data >>= 1;
    data |= bit7;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().set(Flag::C, carry);
    registers.borrow_mut().set(Flag::H, false);
    registers.borrow_mut().set(Flag::N, false);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    };
}

async fn shift_right_hl(registers: Registers, memory: Memory) {
    let address = registers.borrow().get(Bits16::HL);
    let mut data = <Memory as Async<u8>>::get(memory.clone(), address)
        .await
        .unwrap();
    let bit7 = data & BIT7;
    let carry = (data & BIT0) != 0;
    data >>= 1;
    data |= bit7;
    <Memory as Async<u8>>::set(memory.clone(), address, data)
        .await
        .unwrap();
    registers.borrow_mut().set(Flag::C, carry);
    registers.borrow_mut().set(Flag::H, false);
    registers.borrow_mut().set(Flag::N, false);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    };
}

impl ShiftRight {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            ShiftRight::A => shift_right(registers, Bits8::A),
            ShiftRight::B => shift_right(registers, Bits8::B),
            ShiftRight::C => shift_right(registers, Bits8::C),
            ShiftRight::D => shift_right(registers, Bits8::D),
            ShiftRight::E => shift_right(registers, Bits8::E),
            ShiftRight::H => shift_right(registers, Bits8::H),
            ShiftRight::L => shift_right(registers, Bits8::L),
            ShiftRight::HL => shift_right_hl(registers, memory).await,
        };
    }
}

#[cfg(test)]
mod test_shift_right {
    use super::ShiftRight;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_shift_register_right() {
        let src = 0b00000001;
        let expected = 0b00000000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = ShiftRight::A;
        register.borrow_mut().set(Bits8::A, src);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        let result = register.borrow().get(Bits8::A);
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_shift_memory_hl_right() {
        let hl = 0xc008;
        let src = 0b10101010;
        let expected = 0b11010101;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = ShiftRight::HL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow().get_u8(hl).unwrap();
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
    }
}
