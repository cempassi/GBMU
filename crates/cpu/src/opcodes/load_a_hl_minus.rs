use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
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
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits8::A);
        let dst = registers.borrow().get(Bits16::HL);
        <Memory as Async<u8>>::set(memory, dst, data).await.unwrap();
        registers.borrow_mut().set(Bits16::HL, dst.wrapping_sub(1));
    }
}

#[cfg(test)]
mod test_instruction_load_reg_a_hl_minus {
    use super::LoadRegAHLM;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::Area;
    use memory::Memory;

    #[test]
    fn test_load_reg_a_hl_minus() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegAHLM::AHLM;
        let wram_address = Area::Wram.relative(0xc042) as u16;
        register.borrow_mut().set(Bits8::A, 1);
        register.borrow_mut().set(Bits16::HL, wram_address);
        assert_eq!(register.borrow_mut().get(Bits8::A), 1);
        assert_eq!(register.borrow_mut().get(Bits16::HL), wram_address);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            memory.borrow().get_u8(0xc042).unwrap(),
            register.borrow().get(Bits8::A)
        );
        assert_eq!(
            register.borrow().get(Bits16::HL),
            wram_address.wrapping_sub(1)
        );
    }
}
