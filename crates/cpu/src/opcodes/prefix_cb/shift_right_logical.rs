use super::consts::BIT0;
use crate::area::{Bits16, Bits8, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// 11. SRL n
/// Description:
///  Shift n right into Carry. MSB set to 0.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// SRL         A          0xCB3F   8
/// SRL         B          0xCB38   8
/// SRL         C          0xCB39   8
/// SRL         D          0xCB3A   8
/// SRL         E          0xCB3B   8
/// SRL         H          0xCB3C   8
/// SRL         L          0xCB3D   8
/// SRL         (HL)       0xCB3E   16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum ShiftRightLogical {
    A = 0x3f,
    B = 0x38,
    C = 0x39,
    D = 0x3a,
    E = 0x3b,
    H = 0x3c,
    L = 0x3d,
    HL = 0x3e,
}

fn shift_right_logical(registers: Registers, area: Bits8) {
    let mut data = registers.borrow().get(area);
    let carry = (data & BIT0) != 0;
    data >>= 1;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().set(Flag::C, carry);
    registers.borrow_mut().set(Flag::H, false);
    registers.borrow_mut().set(Flag::N, false);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    };
}

async fn shift_right_logical_hl(registers: Registers, memory: Memory) {
    let address = registers.borrow().get(Bits16::HL);
    let mut data = <Memory as Async<u8>>::get(memory.clone(), address)
        .await
        .unwrap();
    let carry = (data & BIT0) != 0;
    data >>= 1;
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

impl ShiftRightLogical {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            ShiftRightLogical::A => shift_right_logical(registers, Bits8::A),
            ShiftRightLogical::B => shift_right_logical(registers, Bits8::B),
            ShiftRightLogical::C => shift_right_logical(registers, Bits8::C),
            ShiftRightLogical::D => shift_right_logical(registers, Bits8::D),
            ShiftRightLogical::E => shift_right_logical(registers, Bits8::E),
            ShiftRightLogical::H => shift_right_logical(registers, Bits8::H),
            ShiftRightLogical::L => shift_right_logical(registers, Bits8::L),
            ShiftRightLogical::HL => shift_right_logical_hl(registers, memory).await,
        };
    }
}

#[cfg(test)]
mod test_shift_right_logical {
    use super::ShiftRightLogical;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_shift_register_right_logically() {
        let src = 0b00000001;
        let expected = 0b00000000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = ShiftRightLogical::A;
        register.borrow_mut().set(Bits8::A, src);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        let result = register.borrow().get(Bits8::A);
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_shift_memory_hl_right_logically() {
        let hl = 0xc008;
        let src = 0b10101010;
        let expected = 0b01010101;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = ShiftRightLogical::HL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow().get_u8(hl).unwrap();
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
    }
}
