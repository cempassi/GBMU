use crate::area::Bits8;
use crate::cpu::Registers;
use crate::futures::{CalculHL, CalculNext, Operation};
use crate::logical::Logical;
use memory::Memory;
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
pub enum Logic {
    AndAA = 0xa7,
    AndAB = 0xa0,
    AndAC = 0xa1,
    AndAD = 0xa2,
    AndAE = 0xa3,
    AndAH = 0xa4,
    AndAL = 0xa5,
    AndAHL = 0xa6,
    AndA8b = 0xe6,
    OrAB = 0xb0,
    OrAC = 0xb1,
    OrAD = 0xb2,
    OrAE = 0xb3,
    OrAH = 0xb4,
    OrAL = 0xb5,
    OrAHL = 0xb6,
    OrAA = 0xb7,
    OrA8b = 0xf6,
    XorAA = 0xaf,
    XorAB = 0xa8,
    XorAC = 0xa9,
    XorAD = 0xaa,
    XorAE = 0xab,
    XorAH = 0xac,
    XorAL = 0xad,
    XorAHL = 0xae,
    XorA8b = 0xee,
    CmpAA = 0xbf,
    CmpAB = 0xb8,
    CmpAC = 0xb9,
    CmpAD = 0xba,
    CmpAE = 0xbb,
    CmpAH = 0xbc,
    CmpAL = 0xbd,
    CmpAHL = 0xbe,
    CmpA8b = 0xfe,
}

impl Logic {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Logic::AndAA => registers.borrow_mut().and(Bits8::A),
            Logic::AndAB => registers.borrow_mut().and(Bits8::B),
            Logic::AndAC => registers.borrow_mut().and(Bits8::C),
            Logic::AndAD => registers.borrow_mut().and(Bits8::D),
            Logic::AndAE => registers.borrow_mut().and(Bits8::E),
            Logic::AndAH => registers.borrow_mut().and(Bits8::H),
            Logic::AndAL => registers.borrow_mut().and(Bits8::L),
            Logic::AndAHL => registers.do_hl(memory, Operation::And).await.unwrap(),
            Logic::AndA8b => registers.do_next(memory, Operation::And).await.unwrap(),
            Logic::OrAB => registers.borrow_mut().or(Bits8::B),
            Logic::OrAC => registers.borrow_mut().or(Bits8::C),
            Logic::OrAD => registers.borrow_mut().or(Bits8::D),
            Logic::OrAE => registers.borrow_mut().or(Bits8::E),
            Logic::OrAH => registers.borrow_mut().or(Bits8::H),
            Logic::OrAL => registers.borrow_mut().or(Bits8::L),
            Logic::OrAA => registers.borrow_mut().or(Bits8::A),
            Logic::OrAHL => registers.do_hl(memory, Operation::Or).await.unwrap(),
            Logic::OrA8b => registers.do_next(memory, Operation::Or).await.unwrap(),
            Logic::XorAA => registers.borrow_mut().xor(Bits8::A),
            Logic::XorAB => registers.borrow_mut().xor(Bits8::B),
            Logic::XorAC => registers.borrow_mut().xor(Bits8::C),
            Logic::XorAD => registers.borrow_mut().xor(Bits8::D),
            Logic::XorAE => registers.borrow_mut().xor(Bits8::E),
            Logic::XorAH => registers.borrow_mut().xor(Bits8::H),
            Logic::XorAL => registers.borrow_mut().xor(Bits8::L),
            Logic::XorAHL => registers.do_hl(memory, Operation::Xor).await.unwrap(),
            Logic::XorA8b => registers.do_next(memory, Operation::Xor).await.unwrap(),
            Logic::CmpAA => registers.borrow_mut().compare(Bits8::A),
            Logic::CmpAB => registers.borrow_mut().compare(Bits8::B),
            Logic::CmpAC => registers.borrow_mut().compare(Bits8::C),
            Logic::CmpAD => registers.borrow_mut().compare(Bits8::D),
            Logic::CmpAE => registers.borrow_mut().compare(Bits8::E),
            Logic::CmpAH => registers.borrow_mut().compare(Bits8::H),
            Logic::CmpAL => registers.borrow_mut().compare(Bits8::L),
            Logic::CmpAHL => registers.do_hl(memory, Operation::Comapre).await.unwrap(),
            Logic::CmpA8b => registers.do_next(memory, Operation::Comapre).await.unwrap(),
        }
    }
}

#[cfg(test)]
mod test_logic_opcodes {
    use super::Logic;
    use crate::area::{Bits8, Bits16, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_and_a_e() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Logic::AndAE;
        register.borrow_mut().set(Bits8::A, 0x4f);
        register.borrow_mut().set(Bits8::E, 0x0f);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x0f);
    }

    #[test]
    fn test_and_next_byte() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Logic::AndA8b;

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert_eq!(register.borrow().get(Flag::H), true);
    }

    #[test]
    fn test_or_a_e() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Logic::OrAE;

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits16::AF), 0x0010);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_or_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Logic::OrA8b;

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits16::AF), 0x3100);
    }
}
