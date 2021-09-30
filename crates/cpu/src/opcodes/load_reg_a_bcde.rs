use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// LD A, nn
/// Description:
///  Put value (nn) into register A.
/// Use with:
///  nn = (BC),(DE)
/// Opcodes:
/// Instruction Parameters  Opcode  Cycles
/// LD          A,(BC)      0x0a    8
/// LD          A,(DE)      0x1a    8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadRegABCDE {
    ABC = 0x0a,
    ADE = 0x1a,
}

impl LoadRegABCDE {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let src = match self {
            LoadRegABCDE::ABC => Bits16::BC,
            LoadRegABCDE::ADE => Bits16::DE,
        };
        let data = memory.borrow().get(registers.borrow().get(src)).unwrap();
        registers.borrow_mut().set(Bits8::A, data)
    }
}

#[cfg(test)]
mod test_instruction_load_reg_a_reg_bc_de {
    use super::LoadRegABCDE;
    use crate::area::{Bits16, Bits8};
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_instruction_load_reg_a_reg_bc() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegABCDE::ABC;
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory
                .borrow()
                .get(register.borrow().get(Bits16::BC))
                .unwrap()
        );
    }

    #[test]
    fn test_instruction_load_reg_a_reg_de() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegABCDE::ADE;
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory
                .borrow()
                .get(register.borrow().get(Bits16::DE))
                .unwrap()
        );
    }
}
