use crate::area::{Bits16, Bits8};
use crate::RegisterBus;
use crate::Registers;
use memory::Async;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// LD n, (HL)
/// Description:
///  Put value (HL) into n.
/// Use with:
///  n = B,C,D,E,H,L,A
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          B,(HL)     0x46   8
/// LD          C,(HL)     0x4E   8
/// LD          D,(HL)     0x56   8
/// LD          E,(HL)     0x5E   8
/// LD          H,(HL)     0x66   8
/// LD          L,(HL)     0x6E   8
/// LD          A,(HL)     0x7E   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadRegHL {
    BHL = 0x46,
    CHL = 0x4E,
    DHL = 0x56,
    EHL = 0x5E,
    HHL = 0x66,
    LHL = 0x6E,
    AHL = 0x7E,
}

impl LoadRegHL {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let dst = match self {
            LoadRegHL::BHL => Bits8::B,
            LoadRegHL::CHL => Bits8::C,
            LoadRegHL::DHL => Bits8::D,
            LoadRegHL::EHL => Bits8::E,
            LoadRegHL::HHL => Bits8::H,
            LoadRegHL::LHL => Bits8::L,
            LoadRegHL::AHL => Bits8::A,
        };
        let src = registers.borrow().get(Bits16::HL);
        let data = <Memory as Async<u8>>::get(memory, src).await.unwrap();
        registers.borrow_mut().set(dst, data);
    }
}

#[cfg(test)]
mod test_instruction_load_reg_hl {
    use super::LoadRegHL;
    use crate::area::{Bits16, Bits8};
    use crate::executor;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_reg_b_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegHL::BHL;
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
