use crate::area::{Bits16, Bits8};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::data::logical::cmp;
use crate::Flags;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// CP n
/// Description:
///  Compare A with n. This is basically an A - n
///  subtraction instruction but the results are thrown
///  away.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),#
/// Flags affected:
///  Z - Set if result is zero. (Set if A = n.)
///  N - Set.
///  H - Set if no borrow from bit 4.
///  C - Set for no borrow. (Set if A < n.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles | Instruction Parameters Opcode Cycles
/// CP          A          0xbf   4        CP          E          0xbb   4
/// CP          B          0xb8   4        CP          H          0xbc   4
/// CP          C          0xb9   4        CP          L          0xbd   4
/// CP          D          0xba   4        CP          (HL)       0xbe   8
/// CP          8b         0xfe   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum CmpRegA {
    AA = 0xbf,
    AB = 0xb8,
    AC = 0xb9,
    AD = 0xba,
    AE = 0xbb,
    AH = 0xbc,
    AL = 0xbd,
    AHL = 0xbe,
    A8b = 0xfe,
}

impl CmpRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data: u8 = match self {
            CmpRegA::AA => registers.borrow().get(Bits8::A),
            CmpRegA::AB => registers.borrow().get(Bits8::B),
            CmpRegA::AC => registers.borrow().get(Bits8::C),
            CmpRegA::AD => registers.borrow().get(Bits8::D),
            CmpRegA::AE => registers.borrow().get(Bits8::E),
            CmpRegA::AH => registers.borrow().get(Bits8::H),
            CmpRegA::AL => registers.borrow().get(Bits8::L),
            CmpRegA::A8b => registers.clone().next_pc(memory.clone()).await.unwrap(),
            CmpRegA::AHL => {
                let src = registers.borrow().get(Bits16::HL);
                <Memory as Async>::get(memory, src).await.unwrap()
            }
        };
        let flag = cmp(data, registers.borrow().get(Bits8::A));
        registers
            .borrow_mut()
            .set(Bits8::F, Flags::into_bytes(flag)[0]);
    }
}

#[cfg(test)]
mod test_instruction_cmp_reg_a_8b {
    use super::CmpRegA;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_cmp_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = CmpRegA::A8b;
        register.borrow_mut().set(Bits8::A, 0x4f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::F), 0xe0);
    }

    #[test]
    fn test_load_cmp_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = CmpRegA::AHL;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::F), 0xe0);
    }

    #[test]
    fn test_load_cmp_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = CmpRegA::AB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::F), 0x30);
    }
}
