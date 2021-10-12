use crate::area::{Bits16, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::opcodes::data::arithmetic::Add;
use crate::opcodes::data::Data;
use num_enum::TryFromPrimitive;

///1. ADD HL,n
/// Description:
///  Add n to HL.
/// Use with:
///  n = BC,DE,HL,SP
/// Flags affected:
///  Z - Not affected.
///  N - Reset.
///  H - Set if carry from bit 11.
///  C - Set if carry from bit 15.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  ADD HL,BC 09 8
///  ADD HL,DE 19 8
///  ADD HL,HL 29 8
///  ADD HL,SP 39 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum AddRegHL {
    BC = 0x09,
    DE = 0x19,
    HL = 0x29,
    SP = 0x39,
}

impl AddRegHL {
    pub async fn exec(self, registers: Registers) {
        let data: Data<u16> = match self {
            AddRegHL::BC => Data::NoCarry(registers.borrow().get(Bits16::BC)),
            AddRegHL::DE => Data::NoCarry(registers.borrow().get(Bits16::DE)),
            AddRegHL::SP => Data::NoCarry(registers.borrow().get(Bits16::SP)),
            AddRegHL::HL => Data::NoCarry(registers.borrow().get(Bits16::HL)),
        };
        let (data, flag) = data.add(registers.borrow().get(Bits16::HL));
        registers.borrow_mut().set(Bits16::HL, data);
        registers.borrow_mut().set(Flag::H, flag.h());
        registers.borrow_mut().set(Flag::C, flag.c());
        registers.borrow_mut().set(Flag::N, flag.n());
    }
}

#[cfg(test)]
mod test_instruction_add_reg_hl {
    use super::AddRegHL;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};

    #[test]
    fn test_load_add_reg_hl_bc() {
        let register = Registers::default();
        let instruction = AddRegHL::BC;
        register.borrow_mut().set(Bits16::BC, 0x4242);
        executor::execute(Box::pin(instruction.exec(register.clone())));
        assert_eq!(register.borrow().get(Bits16::HL), 0x4242);
    }

    #[test]
    fn test_load_add_reg_hl_hl() {
        let register = Registers::default();
        let instruction = AddRegHL::HL;
        register.borrow_mut().set(Bits16::HL, 0x2442);
        executor::execute(Box::pin(instruction.exec(register.clone())));
        assert_eq!(register.borrow().get(Bits16::HL), 0x4884);
    }
}
