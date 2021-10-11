use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// LD (HL), n
/// Description:
///  Put value n into (HL).
/// Use with:
///  n = B,C,D,E,H,L,A
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD          (HL),B     0x70     8
///  LD          (HL),C     0x71     8
///  LD          (HL),D     0x72     8
///  LD          (HL),E     0x73     8
///  LD          (HL),H     0x74     8
///  LD          (HL),L     0x75     8
///  LD          (HL),A     0x77     8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadHLReg {
    HLB = 0x70,
    HLC = 0x71,
    HLD = 0x72,
    HLE = 0x73,
    HLH = 0x74,
    HLL = 0x75,
    HLA = 0x77,
}

impl LoadHLReg {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let src = match self {
            LoadHLReg::HLB => Bits8::B,
            LoadHLReg::HLC => Bits8::C,
            LoadHLReg::HLD => Bits8::D,
            LoadHLReg::HLE => Bits8::E,
            LoadHLReg::HLH => Bits8::H,
            LoadHLReg::HLL => Bits8::L,
            LoadHLReg::HLA => Bits8::A,
        };
        let data = registers.borrow_mut().get(src);
        let dst = registers.borrow().get(Bits16::HL);
        <Memory as Async<u8>>::set(memory, dst, data).await.unwrap();
    }
}

#[cfg(test)]
mod test_instruction_load_hl_reg {
    use super::LoadHLReg;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_hl_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadHLReg::HLB;
        register.borrow_mut().set(Bits16::HL, 0xc042);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::B),
            memory
                .borrow()
                .get_u8(register.borrow().get(Bits16::HL))
                .unwrap()
        );
    }
}
