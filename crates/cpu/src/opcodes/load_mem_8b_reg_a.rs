use crate::area::Bits8;
use crate::nextpc::NextPc;
use crate::opcodes::consts::ADDRESS_OFFSET;
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// LDH (n),A
/// Description:
///  Put A into memory address $FF00+n.
/// Use with:
///  n = one byte immediate value.
/// Opcodes:
/// Instruction Parameters  Opcode Cycles
/// LD          ($FF00+n),A 0xe0   12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadMem8bRegA {
    HnA = 0xe0,
}

impl LoadMem8bRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits8::A);
        let dst: u8 = registers.clone().next_pc(memory.clone()).await.unwrap();
        let dst = dst as u16 + ADDRESS_OFFSET;
        <Memory as Async<u8>>::set(memory, dst, data).await.unwrap()
    }
}

// #[cfg(test)]
// mod test_instruction_load_memory_8bit_reg_a {
//     use super::LoadMem8bRegA;
//     use crate::area::Bits8;
//     use crate::{executor, RegisterBus, Registers};
//     use memory::Memory;
//
//     #[test]
//     fn test_load_memory_8b_reg_a() {
//         let register = Registers::default();
//         let memory = Memory::default();
//         let instruction = LoadMem8bRegA::HnA;
//         executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
//         assert_eq!(
//             register.borrow().get(Bits8::A),
//             memory.borrow().get_u8(0xff31).unwrap()
//         );
//     }
// }
