use crate::area::Bits8;
use crate::opcodes::data::logical::rotate_left;
use crate::opcodes::prefix_cb::consts::{CARRY, NO_CARRY};
use crate::Registers;
use num_enum::TryFromPrimitive;

/// [RLCA | RLA]
/// Description:
/// [RLCA] => Rotate A left. Old bit 7 to Carry flag.
/// [RLA]  => Rotate A left through Carry flag.
/// Flags affected:          Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// RLCA        -/-        0x07   4
/// RLA         -/-        0x17   4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum RotateLeftA {
    RLA = 0x17,
    RLCA = 0x07,
}

impl RotateLeftA {
    pub async fn exec(self, registers: Registers) {
        match self {
            RotateLeftA::RLA => rotate_left(registers, Bits8::A, NO_CARRY),
            RotateLeftA::RLCA => rotate_left(registers, Bits8::A, CARRY),
        }
    }
}

#[cfg(test)]
mod test_rotate_left_register_a {
    use super::RotateLeftA;
    use crate::area::{Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};

    #[test]
    fn test_rotate_left_register_a_no_carry() {
        let src = 0b10001000;
        let expected = 0b00010001;
        let register = Registers::default();
        let instruction = RotateLeftA::RLA;
        register.borrow_mut().set(Bits8::A, src);

        executor::execute(Box::pin(instruction.exec(register.clone())));

        let result = register.borrow().get(Bits8::A);
        let carry = register.borrow_mut().get(Flag::C);
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be true", carry);
        assert_eq!(result, expected);
        assert_eq!(carry, true);
    }

    #[test]
    fn test_rotate_left_register_a_with_carry() {
        let src = 0b10001000;
        let expected = 0b00010000;
        let register = Registers::default();
        let instruction = RotateLeftA::RLCA;
        register.borrow_mut().set(Bits8::A, src);

        executor::execute(Box::pin(instruction.exec(register.clone())));

        let result = register.borrow_mut().get(Bits8::A);
        let carry = register.borrow_mut().get(Flag::C);
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        println!("carry is {} and should be true", carry);
        assert_eq!(result, expected);
        assert_eq!(carry, true);
    }
}
