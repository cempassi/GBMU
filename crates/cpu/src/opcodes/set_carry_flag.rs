use super::super::area::Bits8;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::Flags;
use num_enum::TryFromPrimitive;

/// SCF
/// Description:
///  Set Carry flag.
/// Flags affected:
///  Z - Not affected.
///  N - Reset.
///  H - Reset.
///  C - Set.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// SCF         -/-        0x37   4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum SCF {
    SCF = 0x37,
}

impl SCF {
    pub async fn exec(self, registers: Registers) {
        let mut flag: Flags = Flags::from_bytes([registers.borrow().get(Bits8::F)]);
        flag.set_c(true);
        flag.set_h(false);
        flag.set_n(false);
        registers
            .borrow_mut()
            .set(Bits8::F, Flags::into_bytes(flag)[0]);
    }
}

#[cfg(test)]
mod test_instruction_set_carry_flag {
    use super::SCF;
    use crate::area::Flag;
    use crate::{executor, RegisterBus, Registers};

    #[test]
    fn test_instruction_set_carry_flag() {
        let register = Registers::default();
        let instruction = SCF::SCF;
        register.borrow_mut().set(Flag::C, false);
        executor::execute(Box::pin(instruction.exec(register.clone())));
        assert_eq!(register.borrow().get(Flag::C), true);
    }
}
