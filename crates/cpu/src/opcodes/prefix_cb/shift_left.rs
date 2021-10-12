use super::consts::BIT7;
use crate::area::{Bits16, Bits8, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// 9. SLA n
/// Description:
///  Shift n left into Carry. LSB of n set to 0.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 7 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// SLA         A          0xcb27 8
/// SLA         B          0xcb20 8
/// SLA         C          0xcb21 8
/// SLA         D          0xcb22 8
/// SLA         E          0xcb23 8
/// SLA         H          0xcb24 8
/// SLA         L          0xcb25 8
/// SLA         (HL)       0xcb26 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum ShiftLeft {
    A = 0x27,
    B = 0x20,
    C = 0x21,
    D = 0x22,
    E = 0x23,
    H = 0x24,
    L = 0x25,
    HL = 0x26,
}

fn shift_left(registers: Registers, area: Bits8) {
    let mut data = registers.borrow().get(area);
    let carry = (data & BIT7) != 0;
    data <<= 1;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().set(Flag::C, carry);
    registers.borrow_mut().set(Flag::H, false);
    registers.borrow_mut().set(Flag::N, false);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    };
}

async fn shift_left_hl(registers: Registers, memory: Memory) {
    let address = registers.borrow().get(Bits16::HL);
    let mut data = <Memory as Async<u8>>::get(memory.clone(), address)
        .await
        .unwrap();
    let carry = (data & BIT7) != 0;
    data <<= 1;
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

impl ShiftLeft {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            ShiftLeft::A => shift_left(registers, Bits8::A),
            ShiftLeft::B => shift_left(registers, Bits8::B),
            ShiftLeft::C => shift_left(registers, Bits8::C),
            ShiftLeft::D => shift_left(registers, Bits8::D),
            ShiftLeft::E => shift_left(registers, Bits8::E),
            ShiftLeft::H => shift_left(registers, Bits8::H),
            ShiftLeft::L => shift_left(registers, Bits8::L),
            ShiftLeft::HL => shift_left_hl(registers, memory).await,
        };
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
        let src = 0b10000000;
        let expected = 0b00000000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = ShiftLeft::A;
        register.borrow_mut().set(Bits8::A, src);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        let result = register.borrow().get(Bits8::A);
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
        assert_eq!(register.borrow().get(Flag::Z), true);
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
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow().get_u8(hl).unwrap();
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
    }
}
