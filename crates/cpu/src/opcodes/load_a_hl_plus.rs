use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// LD A,(HLI)
/// Description: Same as: LDI A,(HL)
/// LD A,(HL+)
/// Description: Same as: LDI A,(HL)
/// LDI A,(HL)
/// Description:
/// Put value at address HL into A. Increment HL.
///  Same as: LD A,(HL) - INC HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD A,(HLI) 2A 8
///  LD A,(HL+) 2A 8
///  LDI A,(HL) 2A 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegARegHLP {
    AHLp = 0x2a,
}

impl LoadRegARegHLP {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let hl = registers.borrow().get(Bits16::HL);
        let data = memory.borrow().get(hl).unwrap();
        registers.borrow_mut().set(Bits8::A, data);
        registers.borrow_mut().set(Bits16::HL, hl.wrapping_add(1));
    }
}

#[cfg(test)]
mod test_instruction_load_reg_a_hl_plus {
    use super::LoadRegARegHLP;
    use crate::area::{Bits16, Bits8};
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_reg_a_hlp() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegARegHLP::AHLp;
        let hl = register.borrow().get(Bits16::HL);
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get(hl).unwrap()
        );
        assert_eq!(register.borrow().get(Bits16::HL), hl.wrapping_add(1));
    }
}
