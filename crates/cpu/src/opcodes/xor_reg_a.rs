use super::super::area::Bits8;
use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::data::xor;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// XOR n
/// Description:
///  Logical exclusive OR n with register A, result in A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),8b
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Reset.
/// Opcodes:
/// Instruction Parameters Opcode Cycles | Instruction Parameters Opcode Cycles
/// XOR         A          0xaf   4        XOR         E          0xab   4
/// XOR         B          0xa8   4        XOR         H          0xac   4
/// XOR         C          0xa9   4        XOR         L          0xad   4
/// XOR         D          0xaa   4        XOR         (HL)       0xae   8
/// XOR         8b         0xee   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum XorRegA {
    AA = 0xaf,
    AB = 0xa8,
    AC = 0xa9,
    AD = 0xaa,
    AE = 0xab,
    AH = 0xac,
    AL = 0xad,
    AHL = 0xae,
    A8b = 0xee,
}

impl XorRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data: u8 = match self {
            XorRegA::AA => registers.borrow().get(Bits8::A),
            XorRegA::AB => registers.borrow().get(Bits8::B),
            XorRegA::AC => registers.borrow().get(Bits8::C),
            XorRegA::AD => registers.borrow().get(Bits8::D),
            XorRegA::AE => registers.borrow().get(Bits8::E),
            XorRegA::AH => registers.borrow().get(Bits8::H),
            XorRegA::AL => registers.borrow().get(Bits8::L),
            XorRegA::A8b => registers.clone().next_pc(memory.clone()).await.unwrap(),
            XorRegA::AHL => {
                let src = registers.borrow().get(Bits16::HL);
                <Memory as Async>::get(memory, src).await.unwrap()
            }
        };
        let data = xor(registers.borrow().get(Bits8::A), data);
        registers.borrow_mut().set(Bits16::AF, data);
    }
}

#[cfg(test)]
mod test_instruction_xor_reg_a {
    use super::XorRegA;
    use crate::area::{Bits16, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_xor_reg_a_e() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = XorRegA::AE;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::AF), 0x0010);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_load_xor_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = XorRegA::A8b;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::AF), 0x3100);
    }
}
