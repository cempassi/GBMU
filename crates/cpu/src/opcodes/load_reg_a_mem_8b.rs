use crate::area::Bits8;
use crate::pc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

const ADDRESS_OFFSET: u16 = 0xff00;

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
    pub fn exec(self, registers: Registers, memory: Memory) {
        let src = ADDRESS_OFFSET + registers.borrow_mut().pc.next(memory.clone()).unwrap() as u16;
        let data = memory.borrow_mut().get(src).unwrap();
        registers.borrow_mut().set(Bits8::A, data)
    }
}

#[cfg(test)]
mod test_instruction_reg_a_memory_8bit {
    use super::LoadRegAMem8b;
    use super::ADDRESS_OFFSET;
    use crate::area::Bits8;
    use crate::pc::NextPc;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_memory_reg_a_mem_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegAMem8b::AHn;
        let byte = register.borrow_mut().pc.next(memory.clone()).unwrap();
        let dst = ADDRESS_OFFSET + byte as u16;
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get(dst).unwrap()
        );
    }
}
