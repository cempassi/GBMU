use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// LD (HLD),A
/// Description: Same as: LDD (HL),A
/// LD (HL-),A
/// Description: Same as: LDD (HL),A
/// LDD (HL),A
/// Description:
///  Put A into memory address HL. Decrement HL.
///  Same as: LD (HL),A - DEC HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          (HLD),A    0x32   8
/// LD          (HL-),A    0x32   8
/// LDD         (HL),A     0x32   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegARegHLM {
    AHLM = 0x32,
}

impl LoadRegARegHLM {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let hl = registers.borrow().get(Bits16::HL);
        let data = memory.borrow().get(hl).unwrap();
        registers.borrow_mut().set(Bits8::A, data);
        registers.borrow_mut().set(Bits16::HL, hl - 1);
    }
}

#[cfg(test)]
mod test_instruction_load_reg_a_hl_minus {
    use super::LoadRegARegHLM;
    use crate::area::{Bits16, Bits8};
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_reg_a_hlm() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegARegHLM::AHLm;
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
