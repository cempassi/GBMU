use crate::futures::{Operation, Set};
use crate::registers::{Bits8, Logical as L, Rotation};
use crate::Cpu;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;
use std::fmt;

use super::decode::{Decode, Decoder};

/// RRA
/// Rotate register A right through carry.
///
/// C -> [7 -> 0] -> C

/// RRCA
/// Rotate register A right.
///
/// C -> [7 -> 0] -> [7]

/// LRA
/// Rotate register A left through carry.
///
/// C <- [7 <- 0] <- C

/// LRCA
/// Rotate register A left.
///
/// C <- [7 <- 0] <- [7]

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
#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
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
    RLCA = 0x07,
    RLA = 0x17,
    RRCA = 0x0F,
    RRA = 0x1F,
}

impl Decoder for Logic {
    fn decode(self, cpu: Cpu) -> Decode {
        Box::pin(self.exec(cpu))
    }
}

impl Logic {
    pub async fn exec(self, cpu: Cpu) -> Result<u8, Error> {
        let cycles = match self {
            Logic::RLA => cpu.borrow_mut().registers.left_carry(Bits8::A),
            Logic::RLCA => cpu.borrow_mut().registers.left_nocarry(Bits8::A),
            Logic::RRA => cpu.borrow_mut().registers.right_carry(Bits8::A),
            Logic::RRCA => cpu.borrow_mut().registers.right_nocarry(Bits8::A),
            Logic::AndAA => cpu.borrow_mut().registers.and(Bits8::A),
            Logic::AndAB => cpu.borrow_mut().registers.and(Bits8::B),
            Logic::AndAC => cpu.borrow_mut().registers.and(Bits8::C),
            Logic::AndAD => cpu.borrow_mut().registers.and(Bits8::D),
            Logic::AndAE => cpu.borrow_mut().registers.and(Bits8::E),
            Logic::AndAH => cpu.borrow_mut().registers.and(Bits8::H),
            Logic::AndAL => cpu.borrow_mut().registers.and(Bits8::L),
            Logic::OrAB => cpu.borrow_mut().registers.or(Bits8::B),
            Logic::OrAC => cpu.borrow_mut().registers.or(Bits8::C),
            Logic::OrAD => cpu.borrow_mut().registers.or(Bits8::D),
            Logic::OrAE => cpu.borrow_mut().registers.or(Bits8::E),
            Logic::OrAH => cpu.borrow_mut().registers.or(Bits8::H),
            Logic::OrAL => cpu.borrow_mut().registers.or(Bits8::L),
            Logic::OrAA => cpu.borrow_mut().registers.or(Bits8::A),
            Logic::XorAA => cpu.borrow_mut().registers.xor(Bits8::A),
            Logic::XorAB => cpu.borrow_mut().registers.xor(Bits8::B),
            Logic::XorAC => cpu.borrow_mut().registers.xor(Bits8::C),
            Logic::XorAD => cpu.borrow_mut().registers.xor(Bits8::D),
            Logic::XorAE => cpu.borrow_mut().registers.xor(Bits8::E),
            Logic::XorAH => cpu.borrow_mut().registers.xor(Bits8::H),
            Logic::XorAL => cpu.borrow_mut().registers.xor(Bits8::L),
            Logic::CmpAA => cpu.borrow_mut().registers.compare(Bits8::A),
            Logic::CmpAB => cpu.borrow_mut().registers.compare(Bits8::B),
            Logic::CmpAC => cpu.borrow_mut().registers.compare(Bits8::C),
            Logic::CmpAD => cpu.borrow_mut().registers.compare(Bits8::D),
            Logic::CmpAE => cpu.borrow_mut().registers.compare(Bits8::E),
            Logic::CmpAH => cpu.borrow_mut().registers.compare(Bits8::H),
            Logic::CmpAL => cpu.borrow_mut().registers.compare(Bits8::L),
            Logic::AndAHL => Set::CalculHL(Operation::And).run(cpu).await?,
            Logic::AndA8b => Set::CalculNext(Operation::And).run(cpu).await?,
            Logic::OrAHL => Set::CalculHL(Operation::Or).run(cpu).await?,
            Logic::OrA8b => Set::CalculNext(Operation::Or).run(cpu).await?,
            Logic::XorAHL => Set::CalculHL(Operation::Xor).run(cpu).await?,
            Logic::XorA8b => Set::CalculNext(Operation::Xor).run(cpu).await?,
            Logic::CmpAHL => Set::CalculHL(Operation::Compare).run(cpu).await?,
            Logic::CmpA8b => Set::CalculHL(Operation::Compare).run(cpu).await?,
        };
        Ok(cycles)
    }
}

impl fmt::Display for Logic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Logic::AndAA => write!(f, "And A A"),
            Logic::AndAB => write!(f, "And A B"),
            Logic::AndAC => write!(f, "And A C"),
            Logic::AndAD => write!(f, "And A D"),
            Logic::AndAE => write!(f, "And A E"),
            Logic::AndAH => write!(f, "And A H"),
            Logic::AndAL => write!(f, "And A L"),
            Logic::AndAHL => write!(f, "And A HL"),
            Logic::AndA8b => write!(f, "And A 8b"),
            Logic::OrAB => write!(f, "Or A B"),
            Logic::OrAC => write!(f, "Or A C"),
            Logic::OrAD => write!(f, "Or A D"),
            Logic::OrAE => write!(f, "Or A E"),
            Logic::OrAH => write!(f, "Or A H"),
            Logic::OrAL => write!(f, "Or A L"),
            Logic::OrAHL => write!(f, "Or A HL"),
            Logic::OrAA => write!(f, "Or A A"),
            Logic::OrA8b => write!(f, "Or A 8b"),
            Logic::XorAA => write!(f, "Xor A A"),
            Logic::XorAB => write!(f, "Xor A B"),
            Logic::XorAC => write!(f, "Xor A C"),
            Logic::XorAD => write!(f, "Xor A D"),
            Logic::XorAE => write!(f, "Xor A E"),
            Logic::XorAH => write!(f, "Xor A H"),
            Logic::XorAL => write!(f, "Xor A L"),
            Logic::XorAHL => write!(f, "Xor A HL"),
            Logic::XorA8b => write!(f, "Xor A 8b"),
            Logic::CmpAA => write!(f, "Cmp A A"),
            Logic::CmpAB => write!(f, "Cmp A B"),
            Logic::CmpAC => write!(f, "Cmp A C"),
            Logic::CmpAD => write!(f, "Cmp A D"),
            Logic::CmpAE => write!(f, "Cmp A E"),
            Logic::CmpAH => write!(f, "Cmp A H"),
            Logic::CmpAL => write!(f, "Cmp A L"),
            Logic::CmpAHL => write!(f, "Cmp A HL"),
            Logic::CmpA8b => write!(f, "Cmp A 8b"),
            Logic::RLCA => write!(f, "Rotate Left A"),
            Logic::RLA => write!(f, "Rotate Left Carry A"),
            Logic::RRCA => write!(f, "Rotate Right A"),
            Logic::RRA => write!(f, "Rotate Right Carry A"),
        }
    }
}

#[cfg(test)]
mod test_logic_opcodes {
    use super::Logic;
    use crate::registers::{Bits16, Bits8, Bus, Flag};
    use crate::{Access, Cpu};
    use shared::execute;

    #[test]
    fn test_and_a_e() {
        let cpu = Cpu::default();
        let instruction = Logic::AndAE;
        cpu.borrow_mut().registers.set(Bits8::A, 0x4f);
        cpu.borrow_mut().registers.set(Bits8::E, 0x0f);

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        assert_eq!(cpu.borrow().registers.get(Bits8::A), 0x0f);
        assert!(cpu.borrow().registers.get(Flag::H));
    }

    #[test]
    fn test_and_next_byte() {
        let cpu = Cpu::default();
        let instruction = Logic::AndA8b;

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        assert_eq!(cpu.borrow().registers.get(Bits8::A), 0x00);
        assert!(cpu.borrow().registers.get(Flag::H));
    }

    #[test]
    fn test_or_a_b() {
        let cpu = Cpu::default();
        let instruction = Logic::OrAB;
        cpu.borrow_mut().registers.set(Bits8::A, 0x4A);
        cpu.borrow_mut().registers.set(Bits8::B, 0xF2);

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        assert_eq!(cpu.borrow_mut().registers.get(Bits8::A), 0xFA);
    }

    #[test]
    fn test_or_next_byte() {
        let cpu = Cpu::default();
        let instruction = Logic::OrA8b;

        cpu.borrow_mut().registers.set(Bits8::A, 0x4A);
        cpu.borrow_mut().registers.set(Bits16::PC, 0xc000);
        cpu.memory().borrow_mut().set_u8(0xc000, 0xF2).unwrap();

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        assert_eq!(cpu.borrow().registers.get(Bits8::A), 0xFA);
    }

    #[test]
    fn test_xor_a_d() {
        let cpu = Cpu::default();
        let instruction = Logic::XorAD;
        cpu.borrow_mut().registers.set(Bits8::A, 0x4A);
        cpu.borrow_mut().registers.set(Bits8::D, 0xF2);

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        assert_eq!(cpu.borrow_mut().registers.get(Bits8::A), 0xB8);
    }

    #[test]
    fn test_xor_hl() {
        let cpu = Cpu::default();
        let instruction = Logic::XorAHL;

        cpu.borrow_mut().registers.set(Bits8::A, 0x4A);
        cpu.borrow_mut().registers.set(Bits16::HL, 0xc000);
        cpu.memory().borrow_mut().set_u8(0xc000, 0xF2).unwrap();

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        assert_eq!(cpu.borrow_mut().registers.get(Bits8::A), 0xB8);
    }

    #[test]
    fn test_compare_a_l() {
        let cpu = Cpu::default();
        let instruction = Logic::CmpAL;
        cpu.borrow_mut().registers.set(Bits8::A, 0x4A);
        cpu.borrow_mut().registers.set(Bits8::L, 0xF2);

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        assert_eq!(cpu.borrow().registers.get(Bits8::A), 0x4A);
    }

    #[test]
    fn test_compare_hl() {
        let cpu = Cpu::default();
        let instruction = Logic::CmpAHL;

        cpu.borrow_mut().registers.set(Bits8::A, 0x4A);
        cpu.borrow_mut().registers.set(Bits16::HL, 0xc000);
        cpu.memory().borrow_mut().set_u8(0xc000, 0xF2).unwrap();

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        assert_eq!(cpu.borrow().registers.get(Bits8::A), 0x4A);
    }
}
