use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// LD nn, A
/// Description:
///  Put register A into memory at (nn).
/// Use with:
///  nn = (BC),(DE)
/// Opcodes:
/// Instruction Parameters  Opcode  Cycles
/// LD          (BC),A      0x02    8
/// LD          (DE),A      0x12    8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadBCDERegA {
    BCA = 0x02,
    DEA = 0x12,
}

impl LoadBCDERegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let dst = match self {
            LoadBCDERegA::BCA => Bits16::BC,
            LoadBCDERegA::DEA => Bits16::DE,
        };
        let dst = registers.borrow().get(dst);
        let data = registers.borrow().get(Bits8::A);
        <Memory as Async<u8>>::set(memory, dst, data).await.unwrap();
    }
}

#[cfg(test)]
mod test_instruction_load_bc_de_reg_a {
    use super::LoadBCDERegA;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::{Area, Memory};

    #[test]
    fn test_instruction_load_bc_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadBCDERegA::BCA;
        let wram_address = Area::Wram.relative(0xc042) as u16;
        register.borrow_mut().set(Bits8::A, 1);
        register.borrow_mut().set(Bits16::BC, wram_address);
        assert_eq!(register.borrow_mut().get(Bits8::A), 1);
        assert_eq!(register.borrow_mut().get(Bits16::BC), wram_address);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            memory.borrow().get_u8(0xc042).unwrap(),
            register.borrow().get(Bits8::A)
        );
    }

    #[test]
    fn test_instruction_load_de_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadBCDERegA::DEA;
        let wram_address = Area::Wram.relative(0xc042) as u16;
        register.borrow_mut().set(Bits8::A, 1);
        register.borrow_mut().set(Bits16::DE, wram_address);
        assert_eq!(register.borrow_mut().get(Bits8::A), 1);
        assert_eq!(register.borrow_mut().get(Bits16::DE), wram_address);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            memory.borrow().get_u8(0xc042).unwrap(),
            register.borrow().get(Bits8::A)
        );
    }
}
