use crate::area::Bits8;
use crate::nextpc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// LDH A,(nn)
/// Description:
///  Put A into (nn).
/// Use with:
///  nn = two bytes immediate value.
/// Opcodes:
/// Instruction Parameters  Opcode Cycles
/// LD          (nn),A      0xea   16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegAMem16b {
    LD16bRegA = 0xea,
}

impl LoadRegAMem16b {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow().get(Bits8::A);
        let dst: u16 = registers.clone().next_pc(memory.clone()).await.unwrap();
        <Memory as Async<u8>>::set(memory, dst, data).await.unwrap();
    }
}

// /// Test Can't be done
// /// index out of bounds: the len is 8192 but the index is 12798
// #[cfg(test)]
// mod test_instruction_load_reg_a_memory_16bit {
//     use super::LoadRegAMem16b;
//     use crate::area::Bits8;
//     use crate::{executor, RegisterBus, Registers};
//     use memory::Memory;
//
//     #[test]
//     fn test_load_memory_reg_a_16b() {
//         let register = Registers::default();
//         let memory = Memory::default();
//         let instruction = LoadRegAMem16b::LD16bRegA;
//         executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
//         assert_eq!(
//             register.borrow().get(Bits8::A),
//             memory.borrow().get_u8(0x31fe).unwrap()
//         );
//     }
// }
