use crate::cpu::Registers;
use crate::registers::{
    futures::{CbOperation as Operation, Set},
    Bits8, Shift as S,
};
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::Error;

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

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
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

impl Shift {
    pub async fn exec(self, registers: Registers, memory: Memory) -> Result<u8, Error> {
        let cycles = match self {
            Shift::LB => registers.borrow_mut().shift_left(Bits8::B),
            Shift::LC => registers.borrow_mut().shift_left(Bits8::C),
            Shift::LD => registers.borrow_mut().shift_left(Bits8::D),
            Shift::LE => registers.borrow_mut().shift_left(Bits8::E),
            Shift::LH => registers.borrow_mut().shift_left(Bits8::H),
            Shift::LL => registers.borrow_mut().shift_left(Bits8::L),
            Shift::LA => registers.borrow_mut().shift_left(Bits8::A),
            Shift::LHL => Set::CbHL(Operation::SLeft).run(registers, memory).await?,
            Shift::SB => registers.borrow_mut().swap(Bits8::B),
            Shift::SC => registers.borrow_mut().swap(Bits8::C),
            Shift::SD => registers.borrow_mut().swap(Bits8::D),
            Shift::SE => registers.borrow_mut().swap(Bits8::E),
            Shift::SH => registers.borrow_mut().swap(Bits8::H),
            Shift::SL => registers.borrow_mut().swap(Bits8::L),
            Shift::SA => registers.borrow_mut().swap(Bits8::A),
            Shift::SHL => Set::CbHL(Operation::Swap).run(registers, memory).await?,
            Shift::RAB => registers.borrow_mut().shift_arithmetic(Bits8::B),
            Shift::RAC => registers.borrow_mut().shift_arithmetic(Bits8::C),
            Shift::RAD => registers.borrow_mut().shift_arithmetic(Bits8::D),
            Shift::RAE => registers.borrow_mut().shift_arithmetic(Bits8::E),
            Shift::RAH => registers.borrow_mut().shift_arithmetic(Bits8::H),
            Shift::RAL => registers.borrow_mut().shift_arithmetic(Bits8::L),
            Shift::RAA => registers.borrow_mut().shift_arithmetic(Bits8::A),
            Shift::RAHL => {
                Set::CbHL(Operation::SRArithmetic)
                    .run(registers, memory)
                    .await?
            }
            Shift::RLB => registers.borrow_mut().shift_logic(Bits8::B),
            Shift::RLC => registers.borrow_mut().shift_logic(Bits8::C),
            Shift::RLD => registers.borrow_mut().shift_logic(Bits8::D),
            Shift::RLE => registers.borrow_mut().shift_logic(Bits8::E),
            Shift::RLH => registers.borrow_mut().shift_logic(Bits8::H),
            Shift::RLL => registers.borrow_mut().shift_logic(Bits8::L),
            Shift::RLA => registers.borrow_mut().shift_logic(Bits8::A),
            Shift::RLHL => Set::CbHL(Operation::SRLogic).run(registers, memory).await?,
        };
        Ok(cycles)
    }
}

#[cfg(test)]
mod test_shift_left {
    use super::Shift;
    use crate::registers::{Bits16, Bits8, Bus, Flag};
    use crate::{executor, Registers};
    use memory::Memory;

    #[test]
    fn test_shift_left_register_a() {
        let src = 0b10001000;
        let expected = 0b00010000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Shift::LA;
        register.borrow_mut().set(Bits8::A, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits8::A);
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_shift_left_byte_at_hl() {
        let hl = 0xc008;
        let src = 0b10001000;
        let expected = 0b00010000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Shift::LHL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get_u8(hl).unwrap();
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_shift_right_arithmetic_register_c() {
        let src = 0b1000_1001;
        let expected = 0b1000_0100;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Shift::RAC;
        register.borrow_mut().set(Bits8::C, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits8::C);
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_shift_right_logical_byte_at_hl() {
        let hl = 0xc008;
        let src = 0b1000_1001;
        let expected = 0b0100_0100;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Shift::RLHL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get_u8(hl).unwrap();
        let carry = register.borrow_mut().get(Flag::C);
        assert_eq!(result, expected);
        assert!(carry);
    }

    #[test]
    fn test_swap_register_l() {
        let src = 0b1010_0110;
        let expected = 0b0110_1010;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Shift::SL;
        register.borrow_mut().set(Bits8::L, src);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits8::L);

        println!("Src     : {:08b}", src);
        println!("result  : {:08b}", result);
        println!("expected: {:08b}", expected);
        assert_eq!(result, expected);
    }
}
