use crate::area::{Bits16, Bits8};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::futures::{GetAt, SetAt};
use crate::shift::Shift as S;
use memory::Memory;
use num_enum::TryFromPrimitive;

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

enum Src {
    Register(Bits8),
    Pointer,
}

impl Src {
    pub async fn get(&self, registers: Registers, memory: Memory) -> u8 {
        match self {
            Src::Register(src) => registers.borrow().get(*src),
            Src::Pointer => registers.clone().get_at(memory, Bits16::HL).await.unwrap(),
        }
    }

    pub async fn set(&self, registers: Registers, memory: Memory, data: u8) {
        match self {
            Src::Register(src) => registers.borrow_mut().set(*src, data),
            Src::Pointer => registers
                .clone()
                .set_at(memory, Bits16::HL, data)
                .await
                .unwrap(),
        }
    }
}

enum Type {
    Left,
    Swap,
    Arithmetic,
    Logic,
}

impl Type {
    pub async fn shift(self, src: Src, registers: Registers, memory: Memory) {
        let data = src.get(registers.clone(), memory.clone()).await;
        let data = match self {
            Type::Left => registers.borrow_mut().shift_left(data),
            Type::Swap => registers.borrow_mut().swap(data),
            Type::Arithmetic => registers.borrow_mut().shift_arithmetic(data),
            Type::Logic => registers.borrow_mut().shift_logic(data),
        };
        src.set(registers, memory, data).await;
    }
}

impl Shift {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Shift::LB => Type::Left.shift(Src::Register(Bits8::B), registers, memory),
            Shift::LC => Type::Left.shift(Src::Register(Bits8::C), registers, memory),
            Shift::LD => Type::Left.shift(Src::Register(Bits8::D), registers, memory),
            Shift::LE => Type::Left.shift(Src::Register(Bits8::E), registers, memory),
            Shift::LH => Type::Left.shift(Src::Register(Bits8::H), registers, memory),
            Shift::LL => Type::Left.shift(Src::Register(Bits8::L), registers, memory),
            Shift::LA => Type::Left.shift(Src::Register(Bits8::A), registers, memory),
            Shift::LHL => Type::Left.shift(Src::Pointer, registers, memory),
            Shift::SB => Type::Swap.shift(Src::Register(Bits8::B), registers, memory),
            Shift::SC => Type::Swap.shift(Src::Register(Bits8::C), registers, memory),
            Shift::SD => Type::Swap.shift(Src::Register(Bits8::D), registers, memory),
            Shift::SE => Type::Swap.shift(Src::Register(Bits8::E), registers, memory),
            Shift::SH => Type::Swap.shift(Src::Register(Bits8::H), registers, memory),
            Shift::SL => Type::Swap.shift(Src::Register(Bits8::L), registers, memory),
            Shift::SA => Type::Swap.shift(Src::Register(Bits8::A), registers, memory),
            Shift::SHL => Type::Swap.shift(Src::Pointer, registers, memory),
            Shift::RAB => Type::Arithmetic.shift(Src::Register(Bits8::B), registers, memory),
            Shift::RAC => Type::Arithmetic.shift(Src::Register(Bits8::C), registers, memory),
            Shift::RAD => Type::Arithmetic.shift(Src::Register(Bits8::D), registers, memory),
            Shift::RAE => Type::Arithmetic.shift(Src::Register(Bits8::E), registers, memory),
            Shift::RAH => Type::Arithmetic.shift(Src::Register(Bits8::H), registers, memory),
            Shift::RAL => Type::Arithmetic.shift(Src::Register(Bits8::L), registers, memory),
            Shift::RAA => Type::Arithmetic.shift(Src::Register(Bits8::A), registers, memory),
            Shift::RAHL => Type::Arithmetic.shift(Src::Pointer, registers, memory),
            Shift::RLB => Type::Logic.shift(Src::Register(Bits8::B), registers, memory),
            Shift::RLC => Type::Logic.shift(Src::Register(Bits8::C), registers, memory),
            Shift::RLD => Type::Logic.shift(Src::Register(Bits8::D), registers, memory),
            Shift::RLE => Type::Logic.shift(Src::Register(Bits8::E), registers, memory),
            Shift::RLH => Type::Logic.shift(Src::Register(Bits8::H), registers, memory),
            Shift::RLL => Type::Logic.shift(Src::Register(Bits8::L), registers, memory),
            Shift::RLA => Type::Logic.shift(Src::Register(Bits8::A), registers, memory),
            Shift::RLHL => Type::Logic.shift(Src::Pointer, registers, memory),
        }
        .await;
    }
}

#[cfg(test)]
mod test_shift_left {
    use super::Shift;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
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
        assert_eq!(carry, true);
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
        assert_eq!(carry, true);
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
        assert_eq!(carry, true);
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
        assert_eq!(carry, true);
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
