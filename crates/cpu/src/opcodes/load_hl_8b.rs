use crate::area::Bits16;
use crate::nextpc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use memory::Async;
use num_enum::TryFromPrimitive;

/// 1. LD HL,n
/// Description:
///  Put value n into HL.
/// Use with:
///  n = 8 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          (HL),n     0x36     12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadHL8b {
    HL8b = 0x36,
}

impl LoadHL8b {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let dst = registers.borrow().get(Bits16::HL);
        let data = registers.clone().next_pc(memory.clone()).await.unwrap();
        <Memory as Async>::set(memory,dst, data ).await.unwrap();
    }
}

#[cfg(test)]
mod test_instruction_load_hl_8b {
    use super::LoadHL8b;
    use crate::area::Bits16;
    use crate::{RegisterBus, Registers};
    use memory::Memory;
    use crate::executor;

    #[test]
    fn test_load_hl_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadHL8b::HL8b;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            byte,
            memory
                .borrow()
                .get(register.borrow().get(Bits16::HL))
                .unwrap()
        );
    }
}
