use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// LD (HLI),A
/// Description: Same as: LDI (HL),A
/// LD (HL+),A
/// Description: Same as: LDI (HL),A
/// LDI (HL),A
/// Description:
///  Put A into memory address HL. Increment HL.
///  Same as: LD (HL),A - INC HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          (HLI),A    0x22   8
/// LD          (HL+),A    0x22   8
/// LDI         (HL),A     0x22   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadHLPRegA {
    HLPA = 0x22,
}

impl LoadHLPRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits8::A);
        let dst = registers.borrow().get(Bits16::HL);
        <Memory as Async>::set(memory, dst, data).await.unwrap();
        registers.borrow_mut().set(Bits16::HL, dst.wrapping_add(1));
    }
}

#[cfg(test)]
mod test_instruction_load_reg_a_hl_plus {
    use super::LoadHLPRegA;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::{Area, Memory};

    #[test]
    fn test_reg_a_hlp() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadHLPRegA::HLPA;
        let wram_address = Area::Wram.relative(0xc042) as u16;
        register.borrow_mut().set(Bits16::HL, wram_address);
        assert_eq!(register.borrow_mut().get(Bits16::HL), wram_address);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            memory.borrow().get(0xc042).unwrap(),
            register.borrow().get(Bits8::A)
        );
        assert_eq!(
            register.borrow().get(Bits16::HL),
            wram_address.wrapping_add(1)
        );
    }
}
