use crate::area::{Bits8, Bits16};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::futures::GetAt;
use crate::nextpc::NextPc;
use crate::opcodes::data::arithmetic::{Add, Sub};
use crate::opcodes::data::Data;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// [ADD | ADC] A,n
/// Description:
///  Add [n | n + Carry flag] to A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),8b
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Set if carry from bit 3.
///  C - Set if carry from bit 7.
/// Opcodes:
/// Instruction Parameters Opcode Cycles | Instruction Parameters Opcode Cycles
/// ADD         A,A        0x87   4        ADD         A,E        0x83   4
/// ADD         A,B        0x80   4        ADD         A,H        0x84   4
/// ADD         A,C        0x81   4        ADD         A,L        0x85   4
/// ADD         A,D        0x82   4        ADD         A,(HL)     0x86   8
/// ADD         A,8b       0xc6   8
/// ADC         A,A        0x8f   4        ADC         A,E        0x8b   4
/// ADC         A,B        0x88   4        ADC         A,H        0x8c   4
/// ADC         A,C        0x89   4        ADC         A,L        0x8d   4
/// ADC         A,D        0x8a   4        ADC         A,(HL)     0x8e   8
/// ADC         A,8b       0xce   8

/// [SUB | SBC] n
/// Description:
///  Subtract [n | n + Carry flag] from A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),8b
/// Flags affected:
///  Z - Set if result is zero.
///  N - Set.
///  H - Set if no borrow from bit 4.
///  C - Set if no borrow.
/// Opcodes:
/// Instruction Parameters Opcode Cycles | Instruction Parameters Opcode Cycles
///  SUB        A,A          0x97   4        SBC         A,A        0x9F   4
///  SUB        A,B          0x90   4        SBC         A,B        0x98   4
///  SUB        A,C          0x91   4        SBC         A,C        0x99   4
///  SUB        A,D          0x92   4        SBC         A,D        0x9A   4
///  SUB        A,E          0x93   4        SBC         A,E        0x9B   4
///  SUB        A,H          0x94   4        SBC         A,H        0x9C   4
///  SUB        A,L          0x95   4        SBC         A,L        0x9D   4
///  SUB        A,(HL)       0x96   8        SBC         A,(HL)     0x9E   8
///  SUB        A,8b         0xd6   8        SBC         A,8b       0xDE     ?
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Arithmetic {
    AAA = 0x8f,
    AAB = 0x88,
    AAC = 0x89,
    AAD = 0x8a,
    AAE = 0x8b,
    AAH = 0x8c,
    AAL = 0x8d,
    AAHL = 0x8e,
    AA8b = 0xce,
    AAcA = 0x87,
    AAcB = 0x80,
    AAcC = 0x81,
    AAcD = 0x82,
    AAcE = 0x83,
    AAcH = 0x84,
    AAcL = 0x85,
    AAcHL = 0x86,
    AAc8b = 0xc6,
    SAB = 0x90,
    SAC = 0x91,
    SAD = 0x92,
    SAE = 0x93,
    SAH = 0x94,
    SAL = 0x95,
    SAHL = 0x96,
    SAA = 0x97,
    SAcB = 0x98,
    SAcC = 0x99,
    SAcD = 0x9A,
    SAcE = 0x9B,
    SAcH = 0x9C,
    SAcL = 0x9D,
    SAcHL = 0x9E,
    SAcA = 0x9F,
    SA8b = 0xD6,
    SAc8b = 0xDE,
}

enum Src {
    Register,
    Pointer,
    Next,
}

enum Carry {
    Carry,
    NoCarry,
}

enum Operation {
    Addition(Carry),
    Substraction(Carry),
}

impl Operation {
    pub async fn operate(
        self,
        source: Src,
        registers: Registers,
        memory: Memory,
        src: Option<Bits8>,
    ) {
        let dst = registers.borrow().get(Bits8::A);
        let data = match source {
            Src::Register => registers.borrow().get(src.unwrap()),
            Src::Pointer => registers.clone().get_at(memory, Bits16::HL).await.unwrap(),
            Src::Next => registers.clone().next_pc(memory.clone()).await.unwrap(),
        };
        let data = match self {
            Operation::Addition(Carry::Carry) => Data::Carry(data).add(dst),
            Operation::Addition(Carry::NoCarry) => Data::NoCarry(data).add(dst),
            Operation::Substraction(Carry::Carry) => Data::Carry(data).sub(dst),
            Operation::Substraction(Carry::NoCarry) => Data::NoCarry(data).sub(dst),
        };
        registers.borrow_mut().set(Bits16::AF, data);
    }
}

impl Arithmetic {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Arithmetic::AAA => Operation::Addition(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::A),
            ),
            Arithmetic::AAB => Operation::Addition(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::B),
            ),
            Arithmetic::AAC => Operation::Addition(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::C),
            ),
            Arithmetic::AAD => Operation::Addition(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::D),
            ),
            Arithmetic::AAE => Operation::Addition(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::E),
            ),
            Arithmetic::AAH => Operation::Addition(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::H),
            ),
            Arithmetic::AAL => Operation::Addition(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::L),
            ),
            Arithmetic::AAcA => Operation::Addition(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::A),
            ),
            Arithmetic::AAcB => Operation::Addition(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::B),
            ),
            Arithmetic::AAcC => Operation::Addition(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::C),
            ),
            Arithmetic::AAcD => Operation::Addition(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::D),
            ),
            Arithmetic::AAcE => Operation::Addition(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::E),
            ),
            Arithmetic::AAcH => Operation::Addition(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::H),
            ),
            Arithmetic::AAcL => Operation::Addition(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::L),
            ),
            Arithmetic::AAc8b => {
                Operation::Addition(Carry::Carry).operate(Src::Next, registers, memory, None)
            }
            Arithmetic::AA8b => {
                Operation::Addition(Carry::NoCarry).operate(Src::Next, registers, memory, None)
            }
            Arithmetic::AAHL => {
                Operation::Addition(Carry::NoCarry).operate(Src::Pointer, registers, memory, None)
            }
            Arithmetic::AAcHL => {
                Operation::Addition(Carry::Carry).operate(Src::Pointer, registers, memory, None)
            }
            Arithmetic::SAB => Operation::Substraction(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::B),
            ),
            Arithmetic::SAC => Operation::Substraction(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::C),
            ),
            Arithmetic::SAD => Operation::Substraction(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::D),
            ),
            Arithmetic::SAE => Operation::Substraction(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::E),
            ),
            Arithmetic::SAH => Operation::Substraction(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::H),
            ),
            Arithmetic::SAL => Operation::Substraction(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::L),
            ),
            Arithmetic::SAA => Operation::Substraction(Carry::NoCarry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::A),
            ),
            Arithmetic::SAHL => Operation::Substraction(Carry::NoCarry).operate(
                Src::Pointer,
                registers,
                memory,
                None,
            ),
            Arithmetic::SAcB => Operation::Substraction(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::B),
            ),
            Arithmetic::SAcC => Operation::Substraction(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::C),
            ),
            Arithmetic::SAcD => Operation::Substraction(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::D),
            ),
            Arithmetic::SAcE => Operation::Substraction(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::E),
            ),
            Arithmetic::SAcH => Operation::Substraction(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::H),
            ),
            Arithmetic::SAcL => Operation::Substraction(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::L),
            ),
            Arithmetic::SAcA => Operation::Substraction(Carry::Carry).operate(
                Src::Register,
                registers,
                memory,
                Some(Bits8::A),
            ),
            Arithmetic::SAcHL => {
                Operation::Substraction(Carry::Carry).operate(Src::Pointer, registers, memory, None)
            }
            Arithmetic::SA8b => {
                Operation::Substraction(Carry::NoCarry).operate(Src::Next, registers, memory, None)
            }
            Arithmetic::SAc8b => {
                Operation::Substraction(Carry::Carry).operate(Src::Next, registers, memory, None)
            }
        }
        .await;
    }
}

#[cfg(test)]
mod test_instruction_add_reg_a_8b {
    use super::Arithmetic;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_add_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AA8b;
        register.borrow_mut().set(Bits8::A, 0x4f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x80);
        assert_eq!(register.borrow().get(Flag::H), true);
    }

    #[test]
    fn test_load_add_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAHL;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0xf8);
    }

    #[test]
    fn test_load_add_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_load_adc_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAc8b;
        register.borrow_mut().set(Bits8::A, 0x4f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x81);
        assert_eq!(register.borrow().get(Flag::H), true);
    }

    #[test]
    fn test_load_adc_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAcHL;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0xf9);
    }

    #[test]
    fn test_load_adc_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAcB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x01);
    }

    #[test]
    fn test_load_sub_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SA8b;
        register.borrow_mut().set(Bits8::A, 0x4f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0xe2);
        assert_eq!(register.borrow().get(Flag::H), true);
    }

    #[test]
    fn test_load_sub_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SAHL;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x08);
    }

    #[test]
    fn test_load_sub_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SAB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_load_sbc_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SAcHL;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x07);
    }

    #[test]
    fn test_load_sbc_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SAcB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0xff);
    }
}
