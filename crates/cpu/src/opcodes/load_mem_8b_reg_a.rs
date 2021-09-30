use crate::area::Bits8;
use crate::pc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

const ADDRESS_OFFSET: u16 = 0xff00;

/// LDH (n),A
/// Description:
///  Put A into memory address $FF00+n.
/// Use with:
///  n = one byte immediate value.
/// Opcodes:
/// Instruction Parameters  Opcode Cycles
/// LD          ($FF00+n),A 0xe0   12

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadMem8bRegA {
    HnA = 0xe0,
}

impl LoadMem8bRegA {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits8::A);
        let dst = ADDRESS_OFFSET + registers.borrow_mut().pc.next(memory.clone()).unwrap() as u16;
        memory.borrow_mut().set(dst, data).unwrap()
    }
}

#[cfg(test)]
mod test_instruction_memory_8bit_reg_a {
    use super::LoadMem8bRegA;
    use super::ADDRESS_OFFSET;
    use crate::area::Bits8;
    use crate::pc::NextPc;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_memory_8b_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadMem8bRegA::HnA;
        let byte = register.borrow_mut().pc.next(memory.clone()).unwrap();
        let dst = ADDRESS_OFFSET + byte as u16;
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get(dst).unwrap()
        );
    }
}
