use crate::area::Bits8;
use crate::pc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
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
    pub fn exec(self, registers: Registers, memory: Memory) {
        let first_byte = registers.borrow_mut().pc.next(memory.clone()).unwrap() as u16;
        let second_byte = registers.borrow_mut().pc.next(memory.clone()).unwrap() as u16;
        let src = (second_byte << 8) | first_byte;
        let data = memory.borrow_mut().get(src).unwrap();
        registers.borrow_mut().set(Bits8::A, data);
    }
}

#[cfg(test)]
mod test_instruction_memory_8bit_reg_a {
    use super::LoadMem16bRegA;
    use crate::area::Bits8;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_memory_16b_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadMem16bRegA::LDRegA16b;
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get(0xfe31).unwrap()
        );
    }
}
