use crate::area::Bits8;
use crate::nextpc::NextPc;
use crate::opcodes::consts::ADDRESS_OFFSET;
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// 20. LDH A,(n)
/// Description:
///  Put memory address $FF00+n into A.
/// Use with:
///  n = one byte immediate value.
/// Opcodes:
/// Instruction Parameters  Opcode Cycles
/// LD          A,($FF00+n) 0xf0   12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegAMem8b {
    AHn = 0xf0,
}

impl LoadRegAMem8b {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let src: u8 = registers.clone().next_pc(memory.clone()).await.unwrap();
        let src = src as u16 + ADDRESS_OFFSET;
        let data = <Memory as Async<u8>>::get(memory, src).await.unwrap();
        registers.borrow_mut().set(Bits8::A, data)
    }
}

#[cfg(test)]
mod test_instruction_load_reg_a_memory_8bit {
    use super::LoadRegAMem8b;
    use crate::area::Bits8;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_memory_reg_a_mem_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegAMem8b::AHn;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get_u8(0xff31).unwrap()
        );
    }
}
