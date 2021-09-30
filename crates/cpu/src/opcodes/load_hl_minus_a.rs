use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// LD A,(HLD)
/// Description: Same as: LDD A,(HL)
/// LD A,(HL-)
/// Description: Same as: LDD A,(HL)
/// LDD A,(HL)
/// Description:
///  Put value at address HL into A. Decrement HL.
///  Same as: LD A,(HL) - DEC HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          A,(HLD)    0x3a   8
/// LD          A,(HL-)    0x3a   8
/// LDD         A,(HL)     0x3a   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadHLMRegA {
    HLMA = 0x3a,
}

impl LoadHLMRegA {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let src = registers.borrow().get(Bits16::HL);
        let data = memory.borrow().get(src).unwrap();
        registers.borrow_mut().set(Bits8::A, data);
        registers.borrow_mut().set(Bits16::HL, src.wrapping_sub(1));
    }
}

#[cfg(test)]
mod test_instruction_load_reg_a_hl_minus {
    use super::LoadHLMRegA;
    use crate::area::{Bits16, Bits8};
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_reg_a_hlm() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadHLMRegA::HLMA;
        register.borrow_mut().set(Bits16::HL, 1);
        let hl = register.borrow().get(Bits16::HL);
        assert_eq!(hl, 1);
        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get(hl).unwrap()
        );
        assert_eq!(register.borrow().get(Bits16::HL), 0);
    }
}
