use crate::area::Bits8;
use crate::pc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// LDH A,(nn)
/// Description:
///  Put A into (nn).
/// Use with:
///  nn = two bytes immediate value. Little Endian
/// Opcodes:
/// Instruction Parameters  Opcode Cycles
/// LD          (nn),A      0xea   16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegAMem8b {
    LD16bRegA = 0xea,
}

impl LoadRegAMem8b {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits8::A);
        let first_byte = registers.borrow_mut().pc.next(memory.clone()).unwrap() as u16;
        let second_byte = registers.borrow_mut().pc.next(memory.clone()).unwrap() as u16;
        let dst = (second_byte << 8) | first_byte;
        memory.borrow_mut().set(dst, data);
    }
}

#[cfg(test)]
mod test_instruction_reg_a_memory_16bit {
    use super::LoadRegAMem8b;
    use super::ADDRESS_OFFSET;
    use crate::area::Bits8;
    use crate::pc::NextPc;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_memory_reg_a_16b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadMem16bRegA::LD16bRegA;
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get(0xfe31).unwrap()
        );
    }
}
