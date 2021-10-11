use crate::area::Bits16;
use crate::nextpc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Async;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// LD (nn),SP
/// Description:
///  Put Stack Pointer (SP) at address nn.
/// Use with:
///  nn = two byte immediate address.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD         (nn),SP    0x08   20

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadMem16bRegSP {
    LD16bSP = 0x08,
}

impl LoadMem16bRegSP {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits16::SP);
        let dst: u16 = registers.clone().next_pc(memory.clone()).await.unwrap();
        <Memory as Async<u16>>::set(memory, dst, data)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod test_instruction_load_memory_16bit_reg_sp {
    use super::LoadMem16bRegSP;
    use crate::area::Bits16;
    use crate::executor;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_memory_16bit_reg_sp() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadMem16bRegSP::LD16bSP;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits16::SP),
            memory.borrow().get_u16(0x31fe).unwrap()
        );
    }
}
