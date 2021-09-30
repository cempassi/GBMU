use crate::area::Bits8;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

const C_ADDRESS_OFFSET: u16 = 0xff00;

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
    pub fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits8::A);
        let dst = C_ADDRESS_OFFSET + (registers.borrow().get(Bits8::C) as u16);
        memory.borrow_mut().set(dst, data).unwrap()
    }
}

#[cfg(test)]
mod test_instruction_memory_c_reg_a {
    use super::LoadMemCRegA;
    use super::C_ADDRESS_OFFSET;
    use crate::area::Bits8;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_memory_c_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadMemCRegA::CA;
        let dst = C_ADDRESS_OFFSET + (register.borrow().get(Bits8::C) as u16);
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get(dst).unwrap()
        );
    }
}
