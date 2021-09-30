use crate::area::Bits16;
use crate::pc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
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
    pub fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow_mut().pc.next(memory.clone()).unwrap();
        memory
            .borrow_mut()
            .set(registers.borrow().get(Bits16::HL), data)
            .unwrap()
    }
}

#[cfg(test)]
mod test_instruction_load_hl_8b {
    use super::LoadHL8b;
    use crate::area::Bits16;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_hl_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldhl8b = LoadHL8b::HL8b;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        ldhl8b.exec(register.clone(), memory.clone());
        assert_eq!(
            byte,
            memory
                .borrow()
                .get(register.borrow().get(Bits16::HL))
                .unwrap()
        );
    }
}
