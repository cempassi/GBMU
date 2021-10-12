use crate::area::Bits8;
use crate::nextpc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// 1. LD nn,n
/// Description:
///  Put value n into nn.
/// Use with:
///  nn = B,C,D,E,H,L,A
///  n = 8 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          B,n        0x06   8
/// LD          C,n        0x0e   8
/// LD          D,n        0x16   8
/// LD          E,n        0x1e   8
/// LD          H,n        0x26   8
/// LD          L,n        0x2e   8
/// LD          A,n        0x3e   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone)]
#[repr(u8)]
pub enum LoadR8b {
    B = 0x06,
    C = 0x0e,
    D = 0x16,
    E = 0x1e,
    H = 0x26,
    L = 0x2e,
    A = 0x3e,
}

impl LoadR8b {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.clone().next_pc(memory).await.unwrap();
        let dst = match self {
            LoadR8b::A => Bits8::A,
            LoadR8b::B => Bits8::B,
            LoadR8b::C => Bits8::C,
            LoadR8b::D => Bits8::D,
            LoadR8b::E => Bits8::E,
            LoadR8b::H => Bits8::H,
            LoadR8b::L => Bits8::L,
        };
        registers.borrow_mut().set(dst, data);
    }
}

#[cfg(test)]
mod test_instruction_load_8bit_into_reg {
    use super::LoadR8b;
    use crate::area::Bits8;
    use crate::executor;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = LoadR8b::B;
        let byte = memory.borrow().get_u8(0).unwrap();
        assert_eq!(byte, 0x31);
        let future = ldr8b.exec(register.clone(), memory.clone());
        executor::execute(Box::pin(future));
        assert_eq!(byte, register.borrow().get(Bits8::B));
    }
}