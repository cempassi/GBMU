use super::super::area::Bits8;
use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::data::and;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// AND n
/// Description:
///  Logically AND n with A, result in A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),8b
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Set.
///  C - Reset.
/// Opcodes:
/// Instruction Parameters Opcode Cycles |  Instruction Parameters Opcode Cycles
///  AND        A          0xa7   4         AND        E          0xa3   4
///  AND        B          0xa0   4         AND        H          0xa4   4
///  AND        C          0xa1   4         AND        L          0xa5   4
///  AND        D          0xa2   4         AND        (HL)       0xa6   8
///  AND        8b         0xe6   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum AndRegA {
    AA = 0xa7,
    AB = 0xa0,
    AC = 0xa1,
    AD = 0xa2,
    AE = 0xa3,
    AH = 0xa4,
    AL = 0xa5,
    AHL = 0xa6,
    A8b = 0xe6,
}

impl AndRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data: u8 = match self {
            AndRegA::AA => registers.borrow().get(Bits8::A),
            AndRegA::AB => registers.borrow().get(Bits8::B),
            AndRegA::AC => registers.borrow().get(Bits8::C),
            AndRegA::AD => registers.borrow().get(Bits8::D),
            AndRegA::AE => registers.borrow().get(Bits8::E),
            AndRegA::AH => registers.borrow().get(Bits8::H),
            AndRegA::AL => registers.borrow().get(Bits8::L),
            AndRegA::AHL => {
                let src = registers.borrow().get(Bits16::HL);
                <Memory as Async>::get(memory, src).await.unwrap()
            }
            AndRegA::A8b => registers.clone().next_pc(memory.clone()).await.unwrap(),
        };
        let data = and(registers.borrow().get(Bits8::A), data);
        registers.borrow_mut().set(Bits16::AF, data);
    }
}

#[cfg(test)]
mod test_instruction_and_reg_a {
    use super::AndRegA;
    use crate::area::{Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_and_reg_a_e() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AndRegA::AE;
        register.borrow_mut().set(Bits8::A, 0x4f);
        register.borrow_mut().set(Bits8::E, 0x0f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x0f);
    }

    #[test]
    fn test_load_and_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AndRegA::A8b;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert_eq!(register.borrow().get(Flag::H), true);
    }
}
