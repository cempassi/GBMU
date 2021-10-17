use crate::registers::{Bits16, Bits8, Arithmetic as A};
use crate::bus::Bus;
use crate::cpu::Registers;
use crate::registers::futures::{GetAt, NextPc};
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
    Register(Bits8),
    Pointer,
    Next,
}

impl Src {
    pub async fn data(self, registers: Registers, memory: Memory) -> u8 {
        match self {
            Src::Register(src) => registers.borrow().get(src),
            Src::Pointer => registers.clone().get_at(memory, Bits16::HL).await.unwrap(),
            Src::Next => registers.clone().next_pc(memory.clone()).await.unwrap(),
        }
    }
}

impl Arithmetic {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Arithmetic::AAA => {
                let data = Src::Register(Bits8::A)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, false);
            }
            Arithmetic::AAB => {
                let data = Src::Register(Bits8::B)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, false);
            }
            Arithmetic::AAC => {
                let data = Src::Register(Bits8::C)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, false);
            }
            Arithmetic::AAD => {
                let data = Src::Register(Bits8::D)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, false);
            }
            Arithmetic::AAE => {
                let data = Src::Register(Bits8::E)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, false);
            }
            Arithmetic::AAH => {
                let data = Src::Register(Bits8::H)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, false);
            }
            Arithmetic::AAL => {
                let data = Src::Register(Bits8::L)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, false);
            }
            Arithmetic::AAcA => {
                let data = Src::Register(Bits8::A)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, true);
            }
            Arithmetic::AAcB => {
                let data = Src::Register(Bits8::B)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, true);
            }
            Arithmetic::AAcC => {
                let data = Src::Register(Bits8::C)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, true);
            }
            Arithmetic::AAcD => {
                let data = Src::Register(Bits8::D)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, true);
            }
            Arithmetic::AAcE => {
                let data = Src::Register(Bits8::E)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, true);
            }
            Arithmetic::AAcH => {
                let data = Src::Register(Bits8::H)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, true);
            }
            Arithmetic::AAcL => {
                let data = Src::Register(Bits8::L)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().add(data, true);
            }
            Arithmetic::AAc8b => {
                let data = Src::Next.data(registers.clone(), memory).await;
                registers.borrow_mut().add(data, true);
            }
            Arithmetic::AA8b => {
                let data = Src::Next.data(registers.clone(), memory).await;
                registers.borrow_mut().add(data, false);
            }
            Arithmetic::AAHL => {
                let data = Src::Pointer.data(registers.clone(), memory).await;
                registers.borrow_mut().add(data, false);
            }
            Arithmetic::AAcHL => {
                let data = Src::Pointer.data(registers.clone(), memory).await;
                registers.borrow_mut().add(data, true);
            }
            Arithmetic::SAB => {
                let data = Src::Register(Bits8::B)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, false);
            }
            Arithmetic::SAC => {
                let data = Src::Register(Bits8::C)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, false);
            }
            Arithmetic::SAD => {
                let data = Src::Register(Bits8::D)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, false);
            }
            Arithmetic::SAE => {
                let data = Src::Register(Bits8::E)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, false);
            }
            Arithmetic::SAH => {
                let data = Src::Register(Bits8::H)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, false);
            }
            Arithmetic::SAL => {
                let data = Src::Register(Bits8::L)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, false);
            }
            Arithmetic::SAA => {
                let data = Src::Register(Bits8::A)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, false);
            }
            Arithmetic::SAcB => {
                let data = Src::Register(Bits8::B)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, true);
            }
            Arithmetic::SAcC => {
                let data = Src::Register(Bits8::C)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, true);
            }
            Arithmetic::SAcD => {
                let data = Src::Register(Bits8::D)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, true);
            }
            Arithmetic::SAcE => {
                let data = Src::Register(Bits8::E)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, true);
            }
            Arithmetic::SAcH => {
                let data = Src::Register(Bits8::H)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, true);
            }
            Arithmetic::SAcL => {
                let data = Src::Register(Bits8::L)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, true);
            }
            Arithmetic::SAcA => {
                let data = Src::Register(Bits8::A)
                    .data(registers.clone(), memory)
                    .await;
                registers.borrow_mut().sub(data, true);
            }
            Arithmetic::SAHL => {
                let data = Src::Pointer.data(registers.clone(), memory).await;
                registers.borrow_mut().sub(data, false);
            }
            Arithmetic::SAcHL => {
                let data = Src::Pointer.data(registers.clone(), memory).await;
                registers.borrow_mut().sub(data, true);
            }
            Arithmetic::SA8b => {
                let data = Src::Next.data(registers.clone(), memory).await;
                registers.borrow_mut().sub(data, false);
            }
            Arithmetic::SAc8b => {
                let data = Src::Next.data(registers.clone(), memory).await;
                registers.borrow_mut().sub(data, true);
            }
        }
    }
}

#[cfg(test)]
mod test_arithmetic {
    use super::Arithmetic;
    use crate::registers::{Bits16, Bits8, Flag};
    use crate::{executor, Bus, Registers};
    use memory::Memory;

    #[test]
    fn test_add_next_byte_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AA8b;

        register.borrow_mut().set(Bits8::A, 0x4f);
        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x80);
        assert_eq!(register.borrow().get(Flag::H), true);
    }

    #[test]
    fn test_add_byte_at_address_hl_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAHL;

        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0xf8);
    }

    #[test]
    fn test_add_byte_in_register_b_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAB;

        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_add_next_byte_with_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAc8b;

        register.borrow_mut().set(Bits8::A, 0x4f);
        register.borrow_mut().set(Bits16::PC, 0xc000);
        register.borrow_mut().set(Flag::C, true);
        memory.borrow_mut().set_u8(0xc000, 0x2F).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x7F);
    }

    #[test]
    fn test_add_byte_at_address_hl_with_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAcHL;

        register.borrow_mut().set(Bits8::A, 0x2a);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        register.borrow_mut().set(Flag::C, true);
        memory.borrow_mut().set_u8(0xc008, 0x2d).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x58);
    }

    #[test]
    fn test_add_byte_in_register_c_with_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAcC;

        register.borrow_mut().set(Bits8::A, 0x2B);
        register.borrow_mut().set(Bits8::C, 0xAA);
        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0xD6);
    }

    #[test]
    fn test_sub_next_byte_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SA8b;

        register.borrow_mut().set(Bits8::A, 0x4f);
        register.borrow_mut().set(Bits16::PC, 0xc000);
        register.borrow_mut().set(Flag::C, true);
        memory.borrow_mut().set_u8(0xc000, 0x2F).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x20);
    }

    #[test]
    fn test_sub_byte_at_address_hl_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SAHL;

        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Flag::C, true);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        memory.borrow_mut().set_u8(0xc008, 0xaa).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x4e);
    }

    #[test]
    fn test_sub_byte_in_register_b_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SAB;

        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_sub_byte_at_address_hl_with_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SAcHL;

        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Flag::C, true);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        memory.borrow_mut().set_u8(0xc008, 0xaa).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x4d);
    }

    #[test]
    fn test_sub_byte_in_register_l_with_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SAcL;

        register.borrow_mut().set(Bits8::A, 0xF8);
        register.borrow_mut().set(Bits8::L, 0xAB);
        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        assert_eq!(register.borrow().get(Bits8::A), 0x4C);
    }
}
