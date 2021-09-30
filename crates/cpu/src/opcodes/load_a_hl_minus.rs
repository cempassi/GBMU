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
#[allow(clippy::upper_case_acronyms)]
pub enum LoadRegAHLM {
    AHLM = 0x32,
}

impl LoadRegAHLM {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits8::A);
        let src = registers.borrow().get(Bits16::HL);
        memory.borrow_mut().set(src, data).unwrap();
        registers.borrow_mut().set(Bits16::HL, src.wrapping_sub(1));
    }
}

#[cfg(test)]
mod test_instruction_load_reg_hl_minus_reg_a {
    use super::LoadRegAHLM;
    use crate::area::{Bits16, Bits8};
    use crate::{RegisterBus, Registers};
    use memory::Area;
    use memory::Memory;

    #[test]
    fn test_reg_hlm_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegAHLM::AHLM;
        let wram_address = Area::Wram.relative(0xc042) as u16;
        register.borrow_mut().set(Bits8::A, 1);
        register.borrow_mut().set(Bits16::HL, wram_address);
        assert_eq!(register.borrow_mut().get(Bits8::A), 1);
        assert_eq!(register.borrow_mut().get(Bits16::HL), wram_address);

        instruction.exec(register.clone(), memory.clone());
        assert_eq!(
            memory.borrow().get(0xc042).unwrap(),
            register.borrow().get(Bits8::A)
        );
        assert_eq!(
            register.borrow().get(Bits16::HL),
            wram_address.wrapping_sub(1)
        );
    }
}
