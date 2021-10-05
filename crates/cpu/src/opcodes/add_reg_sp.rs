use crate::area::{Bits16, Bits8};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::data::arithmetic::signed;
use crate::opcodes::data::arithmetic::Add;
use crate::opcodes::data::Data;
use crate::Flags;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// ADD SP,n
/// Description:
///  Add n to Stack Pointer (SP).
/// Use with:
///  n = one byte signed immediate value (r8).
/// Flags affected:
///  Z - Reset.
///  N - Reset.
///  H - Set or reset according to operation.
///  C - Set or reset according to operation.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// ADD         SP,r8      0xe8   16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum AddRegSP {
    SPr8 = 0xe8,
}

impl AddRegSP {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = Data::NoCarry(registers.borrow().get(Bits16::SP));
        let byte = registers.clone().next_pc(memory.clone()).await.unwrap();
        let (data, flag) = data.add(signed(byte));
        registers.borrow_mut().set(Bits16::SP, data);
        registers
            .borrow_mut()
            .set(Bits8::F, Flags::into_bytes(flag)[0]);
    }
}

#[cfg(test)]
mod test_instruction_add_reg_sp {
    use super::AddRegSP;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_add_reg_sp() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegSP::SPr8;
        register.borrow_mut().set(Bits16::SP, 0x4242);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::SP), 0x4273);
    }
}
