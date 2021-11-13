use crate::futures::{CbOperation as Operation, Set};
use crate::registers::{Bits8, Shift as S};
use crate::Cpu;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;
use std::fmt;

use super::decode::{Decode, Decoder};

/// SLA r8 | [HL]
/// Shift Left Arithmetic register r8 | byte pointed to by HL.
///
/// C <- [7 <- 0] <- 0

/// SRA r8 | [HL]
/// Shift Right Arithmetic register r8 | byte pointed to by HL.
///
/// [7] -> [7 -> 0] -> C

/// Swap r8 | [HL]
/// Swap upper 4 bits in register r8 | HL and the lower 4 ones.
///
/// C <- [7 <- 0] <- 0

/// SRL r8 | [HL]
/// Shift Right Logic register r8 | byte pointed to by HL.
///
/// [0] -> [7 -> 0] -> C

/// Cycles: 2
///
/// Bytes: 2
///
/// Flags:
///
/// Z - Set if result is 0.
/// N - Unused
/// H - Unused
/// C - Set according to result.

#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Shift {
    LB = 0x20,
    LC = 0x21,
    LD = 0x22,
    LE = 0x23,
    LH = 0x24,
    LL = 0x25,
    LHL = 0x26,
    LA = 0x27,
    RAB = 0x28,
    RAC = 0x29,
    RAD = 0x2A,
    RAE = 0x2B,
    RAH = 0x2C,
    RAL = 0x2D,
    RAHL = 0x2E,
    RAA = 0x2F,
    SB = 0x30,
    SC = 0x31,
    SD = 0x32,
    SE = 0x33,
    SH = 0x34,
    SL = 0x35,
    SHL = 0x36,
    SA = 0x37,
    RLB = 0x38,
    RLC = 0x39,
    RLD = 0x3A,
    RLE = 0x3B,
    RLH = 0x3C,
    RLL = 0x3D,
    RLHL = 0x3E,
    RLA = 0x3F,
}

impl Decoder for Shift {
    fn decode(self, cpu: Cpu) -> Decode {
        Box::pin(self.exec(cpu))
    }
}

impl Shift {
    pub async fn exec(self, cpu: Cpu) -> Result<u8, Error> {
        let cycles = match self {
            Shift::LB => cpu.borrow_mut().registers.shift_left(Bits8::B),
            Shift::LC => cpu.borrow_mut().registers.shift_left(Bits8::C),
            Shift::LD => cpu.borrow_mut().registers.shift_left(Bits8::D),
            Shift::LE => cpu.borrow_mut().registers.shift_left(Bits8::E),
            Shift::LH => cpu.borrow_mut().registers.shift_left(Bits8::H),
            Shift::LL => cpu.borrow_mut().registers.shift_left(Bits8::L),
            Shift::LA => cpu.borrow_mut().registers.shift_left(Bits8::A),
            Shift::LHL => Set::CbHL(Operation::SLeft).run(cpu).await?,
            Shift::SB => cpu.borrow_mut().registers.swap(Bits8::B),
            Shift::SC => cpu.borrow_mut().registers.swap(Bits8::C),
            Shift::SD => cpu.borrow_mut().registers.swap(Bits8::D),
            Shift::SE => cpu.borrow_mut().registers.swap(Bits8::E),
            Shift::SH => cpu.borrow_mut().registers.swap(Bits8::H),
            Shift::SL => cpu.borrow_mut().registers.swap(Bits8::L),
            Shift::SA => cpu.borrow_mut().registers.swap(Bits8::A),
            Shift::SHL => Set::CbHL(Operation::Swap).run(cpu).await?,
            Shift::RAB => cpu.borrow_mut().registers.shift_arithmetic(Bits8::B),
            Shift::RAC => cpu.borrow_mut().registers.shift_arithmetic(Bits8::C),
            Shift::RAD => cpu.borrow_mut().registers.shift_arithmetic(Bits8::D),
            Shift::RAE => cpu.borrow_mut().registers.shift_arithmetic(Bits8::E),
            Shift::RAH => cpu.borrow_mut().registers.shift_arithmetic(Bits8::H),
            Shift::RAL => cpu.borrow_mut().registers.shift_arithmetic(Bits8::L),
            Shift::RAA => cpu.borrow_mut().registers.shift_arithmetic(Bits8::A),
            Shift::RAHL => Set::CbHL(Operation::SRArithmetic).run(cpu).await?,
            Shift::RLB => cpu.borrow_mut().registers.shift_logic(Bits8::B),
            Shift::RLC => cpu.borrow_mut().registers.shift_logic(Bits8::C),
            Shift::RLD => cpu.borrow_mut().registers.shift_logic(Bits8::D),
            Shift::RLE => cpu.borrow_mut().registers.shift_logic(Bits8::E),
            Shift::RLH => cpu.borrow_mut().registers.shift_logic(Bits8::H),
            Shift::RLL => cpu.borrow_mut().registers.shift_logic(Bits8::L),
            Shift::RLA => cpu.borrow_mut().registers.shift_logic(Bits8::A),
            Shift::RLHL => Set::CbHL(Operation::SRLogic).run(cpu).await?,
        };
        Ok(cycles)
    }
}
impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shift::LB => write!(f, "Shift L B"),
            Shift::LC => write!(f, "Shift L C"),
            Shift::LD => write!(f, "Shift L D"),
            Shift::LE => write!(f, "Shift L E"),
            Shift::LH => write!(f, "Shift L H"),
            Shift::LL => write!(f, "Shift L L"),
            Shift::LHL => write!(f, "Shift L [HL]"),
            Shift::LA => write!(f, "Shift L A"),
            Shift::RAB => write!(f, "Shift R B (Arithmetic)"),
            Shift::RAC => write!(f, "Shift R C (Arithmetic)"),
            Shift::RAD => write!(f, "Shift R D (Arithmetic)"),
            Shift::RAE => write!(f, "Shift R E (Arithmetic)"),
            Shift::RAH => write!(f, "Shift R H (Arithmetic)"),
            Shift::RAL => write!(f, "Shift R L (Arithmetic)"),
            Shift::RAHL => write!(f, "Shift R [HL] (Arithmetic)"),
            Shift::RAA => write!(f, "Shift R A (Arithmetic)"),
            Shift::SB => write!(f, "Swap B"),
            Shift::SC => write!(f, "Swap C"),
            Shift::SD => write!(f, "Swap D"),
            Shift::SE => write!(f, "Swap E"),
            Shift::SH => write!(f, "Swap H"),
            Shift::SL => write!(f, "Swap L"),
            Shift::SHL => write!(f, "Swap [HL]"),
            Shift::SA => write!(f, "Swap A"),
            Shift::RLB => write!(f, "Shift R B (Logical)"),
            Shift::RLC => write!(f, "Shift R C (Logical)"),
            Shift::RLD => write!(f, "Shift R D (Logical)"),
            Shift::RLE => write!(f, "Shift R E (Logical)"),
            Shift::RLH => write!(f, "Shift R H (Logical)"),
            Shift::RLL => write!(f, "Shift R L (Logical)"),
            Shift::RLHL => write!(f, "Shift R [ HL] (Logical)"),
            Shift::RLA => write!(f, "Shift R A (Logical)"),
        }
    }
}

#[cfg(test)]
mod test_shift_left {
    use super::Shift;
    use crate::registers::{Bits16, Bits8, Bus, Flag};
    use crate::{Access, Cpu};
    use shared::execute;

    #[test]
    fn test_shift_left_register_a() {
        let src = 0b10001000;
        let expected = 0b00010000;
        let cpu = Cpu::default();
        let instruction = Shift::LA;
        cpu.borrow_mut().registers.set(Bits8::A, src);

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let result = cpu.borrow().registers.get(Bits8::A);
        let carry = cpu.borrow().registers.get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_shift_left_byte_at_hl() {
        let hl = 0xc008;
        let src = 0b10001000;
        let expected = 0b00010000;
        let cpu = Cpu::default();
        let instruction = Shift::LHL;
        cpu.borrow_mut().registers.set(Bits16::HL, hl);
        cpu.memory().borrow_mut().set_u8(hl, src).unwrap();

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let result = cpu.memory().borrow_mut().get_u8(hl).unwrap();
        let carry = cpu.borrow().registers.get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_shift_right_arithmetic_register_c() {
        let src = 0b1000_1001;
        let expected = 0b1000_0100;
        let cpu = Cpu::default();
        let instruction = Shift::RAC;
        cpu.borrow_mut().registers.set(Bits8::C, src);

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let result = cpu.borrow().registers.get(Bits8::C);
        let carry = cpu.borrow().registers.get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_shift_right_logical_byte_at_hl() {
        let hl = 0xc008;
        let src = 0b1000_1001;
        let expected = 0b0100_0100;
        let cpu = Cpu::default();
        let instruction = Shift::RLHL;
        cpu.borrow_mut().registers.set(Bits16::HL, hl);
        cpu.memory().borrow_mut().set_u8(hl, src).unwrap();

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let result = cpu.memory().borrow_mut().get_u8(hl).unwrap();
        let carry = cpu.borrow().registers.get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_swap_register_l() {
        let src = 0b1010_0110;
        let expected = 0b0110_1010;
        let cpu = Cpu::default();
        let instruction = Shift::SL;
        cpu.borrow_mut().registers.set(Bits8::L, src);

        execute(Box::pin(instruction.exec(cpu.clone()))).unwrap();

        let result = cpu.borrow().registers.get(Bits8::L);

        println!("Src     : {:08b}", src);
        println!("result  : {:08b}", result);
        println!("expected: {:08b}", expected);
        assert_eq!(result, expected);
    }
}
