use super::super::area::Bits8;
use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::data::or;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// OR n
/// Description:
///  Logical OR n with register A, result in A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),8b
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Reset.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// OR          A          0xb7   4
/// OR          B          0xb0   4
/// OR          C          0xb1   4
/// OR          D          0xb2   4
/// OR          E          0xb3   4
/// OR          H          0xb4   4
/// OR          L          0xb5   4
/// OR          (HL)       0xb6   8
/// OR          8b         0xf6   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum OrRegA {
    AB = 0xb0,
    AC = 0xb1,
    AD = 0xb2,
    AE = 0xb3,
    AH = 0xb4,
    AL = 0xb5,
    AHL = 0xb6,
    AA = 0xb7,
    A8b = 0xf6,
}

impl OrRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data: u8 = match self {
            OrRegA::AA => registers.borrow().get(Bits8::A),
            OrRegA::AB => registers.borrow().get(Bits8::B),
            OrRegA::AC => registers.borrow().get(Bits8::C),
            OrRegA::AD => registers.borrow().get(Bits8::D),
            OrRegA::AE => registers.borrow().get(Bits8::E),
            OrRegA::AH => registers.borrow().get(Bits8::H),
            OrRegA::AL => registers.borrow().get(Bits8::L),
            OrRegA::A8b => registers.clone().next_pc(memory.clone()).await.unwrap(),
            OrRegA::AHL => {
                let src = registers.borrow().get(Bits16::HL);
                <Memory as Async<u8>>::get(memory, src).await.unwrap()
            }
        };
        let data = or(registers.borrow().get(Bits8::A), data);
        registers.borrow_mut().set(Bits16::AF, data);
    }
}

#[cfg(test)]
mod test_instruction_or_reg_a {
    use super::OrRegA;
    use crate::area::{Bits16, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_or_reg_a_e() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = OrRegA::AE;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::AF), 0x0010);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_load_or_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = OrRegA::A8b;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::AF), 0x3100);
    }
}
