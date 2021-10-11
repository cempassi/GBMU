use crate::area::Bits8;
use crate::opcodes::data::rotate_right;
use crate::opcodes::prefix_cb::consts::{CARRY, NO_CARRY};
use crate::Registers;
use num_enum::TryFromPrimitive;

/// [RRCA | RRA]
/// Description:
/// [RRCA] => Rotate A right. Old bit 0 to Carry flag.
/// [RRA]  => Rotate A right through Carry flag.
/// Flags affected:          Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// RRCA        -/-        0x0F   4
/// RRA         -/-        0x1F   4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum RotateRightA {
    RRA = 0x1f,
    RRCA = 0x0f,
}

impl RotateRightA {
    pub async fn exec(self, registers: Registers) {
        match self {
            RotateRightA::RRA => rotate_right(registers, Bits8::A, NO_CARRY),
            RotateRightA::RRCA => rotate_right(registers, Bits8::A, CARRY),
        }
    }
}

#[cfg(test)]
mod test_rotate_right_register_a {
    use super::RotateRightA;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_rotate_right_register_a_no_carry() {
        let src = 0b00010001;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateRightA::RRA;
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
    fn test_rotate_right_register_a_with_carry() {
        let src = 0b00010000;
        let expected = 0b10001000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RotateRightA::RRCA;
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
}
