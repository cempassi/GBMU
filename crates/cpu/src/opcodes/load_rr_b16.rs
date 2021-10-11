use crate::area::Bits16;
use crate::nextpc::NextPc;
use crate::{RegisterBus, Registers};
use memory::Memory;
use num_enum::TryFromPrimitive;

///1. LD n,nn
/// Description:
///  Put value nn into n.
/// Use with:
///  n = BC,DE,HL,SP
///  nn = 16 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD BC,nn 01 12
///  LD DE,nn 11 12
///  LD HL,nn 21 12
///  LD SP,nn 31 12

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRR16b {
    BC = 0x01,
    DE = 0x11,
    HL = 0x21,
    SP = 0x31,
}

impl LoadRR16b {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data: u16 = registers.clone().next_pc(memory.clone()).await.unwrap();
        let dst = match self {
            LoadRR16b::BC => Bits16::BC,
            LoadRR16b::DE => Bits16::DE,
            LoadRR16b::HL => Bits16::HL,
            LoadRR16b::SP => Bits16::SP,
        };
        registers.borrow_mut().set(dst, data)
    }
}

#[cfg(test)]
mod test_instruction_load_double_register_b16 {
    use super::LoadRR16b;
    use crate::area::Bits16;
    use crate::executor;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_instruction_load_bc_b16() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRR16b::BC;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(0x31fe, register.borrow().get(Bits16::BC));
    }
}
