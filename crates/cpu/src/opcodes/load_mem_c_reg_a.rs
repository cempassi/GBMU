use crate::area::Bits8;
use crate::opcodes::consts::ADDRESS_OFFSET;
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// 6. LD (C),A
/// Description:
/// Put A into address $FF00 + register C.
/// Opcodes:
/// Instruction Parameters      Opcode  Cycles
/// LD          ($FF00+C),A     0xe2    8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadMemCRegA {
    CA = 0xe2,
}

impl LoadMemCRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits8::A);
        let dst = ADDRESS_OFFSET + (registers.borrow().get(Bits8::C) as u16);
        <Memory as Async<u8>>::set(memory, dst, data).await.unwrap()
    }
}

#[cfg(test)]
mod test_instruction_memory_c_reg_a {
    use super::LoadMemCRegA;
    use super::ADDRESS_OFFSET;
    use crate::area::Bits8;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_memory_c_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadMemCRegA::CA;
        let dst = ADDRESS_OFFSET + (register.borrow().get(Bits8::C) as u16);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get_u8(dst).unwrap()
        );
    }
}
