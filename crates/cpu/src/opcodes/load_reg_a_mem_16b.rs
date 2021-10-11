use crate::area::Bits8;
use crate::nextpc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// LDH A, (nn)
/// Description:
///  Put (nn) into A.
/// Use with:
///  nn = two bytes immediate value.
/// Opcodes:
/// Instruction Parameters  Opcode Cycles
/// LD          A,(nn)      0xfa   16

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadMem16bRegA {
    LDRegA16b = 0xfa,
}

impl LoadMem16bRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let src: u16 = registers.clone().next_pc(memory.clone()).await.unwrap();
        let data = <Memory as Async<u8>>::get(memory, src).await.unwrap();
        registers.borrow_mut().set(Bits8::A, data);
    }
}

#[cfg(test)]
mod test_instruction_memory_8bit_reg_a {
    use super::LoadMem16bRegA;
    use crate::area::Bits8;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_memory_16b_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadMem16bRegA::LDRegA16b;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get_u8(0x31fe).unwrap()
        );
    }
}
