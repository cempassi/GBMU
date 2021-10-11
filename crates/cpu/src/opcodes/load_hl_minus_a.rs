use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
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
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let src = registers.borrow().get(Bits16::HL);
        let data = <Memory as Async<u8>>::get(memory, src).await.unwrap();
        registers.borrow_mut().set(Bits8::A, data);
        registers.borrow_mut().set(Bits16::HL, src.wrapping_sub(1));
    }
}

#[cfg(test)]
mod test_instruction_load_hl_minus_reg_a {
    use super::LoadHLMRegA;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_hl_minus_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadHLMRegA::HLMA;
        register.borrow_mut().set(Bits16::HL, 1);
        let hl = register.borrow().get(Bits16::HL);
        assert_eq!(hl, 1);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::A),
            memory.borrow().get_u8(hl).unwrap()
        );
        assert_eq!(register.borrow().get(Bits16::HL), 0);
    }
}
