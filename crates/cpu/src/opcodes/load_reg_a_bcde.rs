use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
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
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let src = match self {
            LoadRegABCDE::ABC => Bits16::BC,
            LoadRegABCDE::ADE => Bits16::DE,
        };
        let src = registers.borrow().get(src);
        let data = <Memory as Async<u8>>::get(memory, src).await.unwrap();
        registers.borrow_mut().set(Bits8::A, data)
    }
}

#[cfg(test)]
mod test_instruction_load_reg_a_reg_bc_de {
    use super::LoadRegABCDE;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_instruction_load_reg_a_reg_bc() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegABCDE::ABC;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory
                .borrow()
                .get_u8(register.borrow().get(Bits16::BC))
                .unwrap()
        );
    }

    #[test]
    fn test_instruction_load_reg_a_reg_de() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegABCDE::ADE;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory
                .borrow()
                .get_u8(register.borrow().get(Bits16::DE))
                .unwrap()
        );
    }
}
