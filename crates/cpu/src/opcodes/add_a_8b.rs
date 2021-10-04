use super::super::area::Bits8;
use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::add8;
use crate::Flags;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// ADD A,n
/// Description:
///  Add n to A.
/// Use with:
///  n = A,B,C,D,E,H,L
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Set if carry from bit 3.
///  C - Set if carry from bit 7.
/// Opcodes:
/// Instruction Parameters Opcode Cycles | Instruction Parameters Opcode Cycles
/// ADD         A,A        0x87   4        ADD         A,E        83     4
/// ADD         A,B        0x80   4        ADD         A,H        84     4
/// ADD         A,C        0x81   4        ADD         A,L        85     4
/// ADD         A,D        0x82   4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum AddRegA8b {
    AB = 0x80,
    AC = 0x81,
    AD = 0x82,
    AE = 0x83,
    AH = 0x84,
    AL = 0x85,
    AHL = 0x86,
    AA = 0x87,
    A8b = 0xc6,
}

impl AddRegA8b {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data: u8 = match self {
            AddRegA8b::AA => registers.borrow().get(Bits8::A),
            AddRegA8b::AB => registers.borrow().get(Bits8::B),
            AddRegA8b::AC => registers.borrow().get(Bits8::C),
            AddRegA8b::AD => registers.borrow().get(Bits8::D),
            AddRegA8b::AE => registers.borrow().get(Bits8::E),
            AddRegA8b::AH => registers.borrow().get(Bits8::H),
            AddRegA8b::AL => registers.borrow().get(Bits8::L),
            AddRegA8b::AHL => {
                let src = registers.borrow().get(Bits16::HL);
                <Memory as Async>::get(memory, src).await.unwrap()
            }
            AddRegA8b::A8b => registers.clone().next_pc(memory.clone()).await.unwrap(),
        };
        let (data, flag) = add8(data, registers.borrow().get(Bits8::A), false);
        registers.borrow_mut().set(Bits8::A, data);
        registers
            .borrow_mut()
            .set(Bits8::F, Flags::into_bytes(flag)[0]);
    }
}

#[cfg(test)]
mod test_instruction_add_reg_a_8b {
    use super::AddRegA8b;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_add_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegA8b::A8b;
        register.borrow_mut().set(Bits8::A, 0x4f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x80);
        assert_eq!(register.borrow().get(Flag::H), true);
    }

    #[test]
    fn test_load_add_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegA8b::A8b;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x29);
        assert_eq!(register.borrow().get(Flag::C), true);
    }

    #[test]
    fn test_load_add_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegA8b::AB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }
}
