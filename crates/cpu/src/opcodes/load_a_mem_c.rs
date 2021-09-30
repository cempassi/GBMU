use crate::area::Bits8;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

const C_ADDRESS_OFFSET: u16 = 0xff00;

/// LD A,(C)
/// Description:
///  Put value at address $FF00 + register C into A.
///  Same as: LD A,($FF00+C)
///  Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD A,(C) F2 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegAMemC {
    AC = 0xf2,
}

impl LoadRegAMemC {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let src = C_ADDRESS_OFFSET + (registers.borrow().get(Bits8::C) as u16);
        let data = memory.borrow_mut().get(src).unwrap();
        registers.borrow_mut().set(Bits8::A, data)
    }
}

#[cfg(test)]
mod test_instruction_load_reg_a_memory_c {
    use super::LoadRegAMemC;
    use crate::area::Bits8;
    use crate::opcodes::load_a_mem_c::C_ADDRESS_OFFSET;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_reg_a_memory_c() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegAMemC::AC;
        let src = C_ADDRESS_OFFSET + (register.borrow().get(Bits8::C) as u16);
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get(src).unwrap()
        );
    }
}
