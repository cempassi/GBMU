use crate::area::{Bits16, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::data::arithmetic::signed;
use crate::opcodes::data::{Add, Data};
use memory::Memory;
use num_enum::TryFromPrimitive;

/// LD HL,SP+n
/// Description: Same as: LDHL SP,n.
/// LD HL SP,n
/// Description:
///  Put SP + n effective address into HL.
/// Use with:
///  n = one byte signed immediate value.
/// Flags affected:
///  Z - Reset.
///  N - Reset.
///  H - Set or reset according to operation.
///  C - Set or reset according to operation.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          HL SP,n    0xf8   12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegHLRegSPr8 {
    LDSPr8 = 0xf8,
}

/// IL Y A UN PB CAR suite a signed la data passe en u16 et le add<u16> ne set pas les meme flags
/// que le add<u8> or il faut les flags du add<u8> la.......
/// NEED TO CHECK LATER
impl LoadRegHLRegSPr8 {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let src = Data::NoCarry(registers.borrow().get(Bits16::SP));
        let data = signed(registers.clone().next_pc(memory.clone()).await.unwrap());
        let (data, flag) = src.add(data);
        registers.borrow_mut().set(Bits16::HL, data);
        registers.borrow_mut().set(Flag::Z, false);
        registers.borrow_mut().set(Flag::N, false);
        registers.borrow_mut().set(Flag::C, flag.c());
        registers.borrow_mut().set(Flag::H, flag.h());
        todo!();
    }
}

// #[cfg(test)]
// mod test_instruction_load_reg_hl_reg_sp_r8 {
//     use super::LoadRegHLRegSPr8;
//     use crate::area::Bits16;
//     use crate::{executor, RegisterBus, Registers};
//     use memory::Memory;
//
//     #[test]
//     fn test_load_reg_hl_reg_sp_r8() {
//         let register = Registers::default();
//         let memory = Memory::default();
//         let instruction = LoadRegHLRegSPr8::LDSPr8;
//         register.borrow_mut().set(Bits16::SP, 0x4242);
//         executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
//         assert_eq!(register.borrow().get(Bits16::HL), 0x4273);
//     }
// }
