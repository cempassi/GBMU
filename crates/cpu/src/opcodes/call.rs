use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::Cpu;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// CALL nn
/// Description:
///  Push address of next instruction onto stack and then jump to address nn.
/// Use with:
///  nn = two byte immediate value. (LS byte first.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  CALL nn CD 12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Call {
    CALL = 0xcd,
}

impl Call {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = memory
            .borrow_mut()
            .get_u16(registers.borrow().get(Bits16::PC))
            .unwrap();
        Cpu::push(registers.clone(), memory.clone(), data)
            .await
            .unwrap();
        Cpu::jump(registers.clone(), data);
    }
}

#[cfg(test)]
mod test_instruction_call {
    use super::Call;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_call_instruction() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Call::CALL;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::PC), 0x31fe);
    }
}
