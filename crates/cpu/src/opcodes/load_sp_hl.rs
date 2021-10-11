use crate::area::Bits16;
use crate::RegisterBus;
use crate::Registers;
use num_enum::TryFromPrimitive;

/// LD SP,HL
/// Description:
///  Put HL into Stack Pointer (SP).
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          SP,HL      0xf9   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadRegSPRegHL {
    SPHL = 0xf9,
}

impl LoadRegSPRegHL {
    pub async fn exec(self, registers: Registers) {
        let data = registers.borrow().get(Bits16::HL);
        registers.borrow_mut().set(Bits16::SP, data)
    }
}

#[cfg(test)]
mod test_instruction_load_hl_reg {
    use super::LoadRegSPRegHL;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};

    #[test]
    fn test_load_reg_sp_reg_hl() {
        let register = Registers::default();
        let instruction = LoadRegSPRegHL::SPHL;
        register.borrow_mut().set(Bits16::HL, 0x4242);
        executor::execute(Box::pin(instruction.exec(register.clone())));
        assert_eq!(register.borrow().get(Bits16::SP), 0x4242);
    }
}
