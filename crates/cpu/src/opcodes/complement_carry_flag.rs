use super::super::area::Bits8;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::Flags;
use num_enum::TryFromPrimitive;

/// CCF
/// Description:
///  Complement carry flag.
///  If C flag is set, then reset it.
///  If C flag is reset, then set it.
/// Flags affected:
///  Z - Not affected.
///  N - Reset.
///  H - Reset.
///  C - Complemented.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// CCF         -/-        0x3F   4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum CCF {
    CCF = 0x3f,
}

impl CCF {
    pub async fn exec(self, registers: Registers) {
        let mut flag: Flags = Flags::from_bytes([registers.borrow().get(Bits8::F)]);
        if flag.c() {
            flag.set_c(false);
        } else {
            flag.set_c(true);
        }
        flag.set_h(false);
        flag.set_n(false);
        registers
            .borrow_mut()
            .set(Bits8::F, Flags::into_bytes(flag)[0]);
    }
}

#[cfg(test)]
mod test_instruction_complement_carry_flag {
    use super::CCF;
    use crate::area::Flag;
    use crate::{executor, RegisterBus, Registers};

    #[test]
    fn test_instruction_complement_carry_flag() {
        let register = Registers::default();
        let instruction = CCF::CCF;
        register.borrow_mut().set(Flag::C, true);
        executor::execute(Box::pin(instruction.exec(register.clone())));
        assert_eq!(register.borrow().get(Flag::C), false);
    }
}
