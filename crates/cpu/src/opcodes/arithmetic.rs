use super::decode::{Decode, Decoder};
use crate::cpu::Registers;
use crate::registers::{
    futures::{Operation, Set},
    Arithmetic as A, Bits8, Complement, IncDec,
};
use memory::Memory;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;

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
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
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
    IncB = 0x04,
    IncD = 0x14,
    IncH = 0x24,
    IncHL = 0x34,
    DecB = 0x05,
    DecD = 0x15,
    DecH = 0x25,
    DecHL = 0x35,
    IncC = 0x0C,
    IncE = 0x1C,
    IncL = 0x2C,
    IncA = 0x3C,
    DecC = 0x0D,
    DecE = 0x1D,
    DecL = 0x2D,
    DecA = 0x3D,
    DAA = 0x26,
    SCF = 0x37,
    CPL = 0x2F,
    CCF = 0x3F,
}

impl Decoder for Arithmetic {
    fn decode(self, registers: Registers, memory: Memory) -> Decode {
        Box::pin(self.exec(registers, memory))
    }
}

impl Arithmetic {
    pub async fn exec(self, registers: Registers, memory: Memory) -> Result<u8, Error> {
        let cycles = match self {
            Arithmetic::AAA => registers.borrow_mut().add(Bits8::A, false),
            Arithmetic::AAB => registers.borrow_mut().add(Bits8::B, false),
            Arithmetic::AAC => registers.borrow_mut().add(Bits8::C, false),
            Arithmetic::AAD => registers.borrow_mut().add(Bits8::D, false),
            Arithmetic::AAE => registers.borrow_mut().add(Bits8::E, false),
            Arithmetic::AAH => registers.borrow_mut().add(Bits8::H, false),
            Arithmetic::AAL => registers.borrow_mut().add(Bits8::L, false),
            Arithmetic::AAcA => registers.borrow_mut().add(Bits8::A, true),
            Arithmetic::AAcB => registers.borrow_mut().add(Bits8::B, true),
            Arithmetic::AAcC => registers.borrow_mut().add(Bits8::C, true),
            Arithmetic::AAcD => registers.borrow_mut().add(Bits8::D, true),
            Arithmetic::AAcE => registers.borrow_mut().add(Bits8::E, true),
            Arithmetic::AAcH => registers.borrow_mut().add(Bits8::H, true),
            Arithmetic::AAcL => registers.borrow_mut().add(Bits8::L, true),
            Arithmetic::SAB => registers.borrow_mut().sub(Bits8::B, false),
            Arithmetic::SAC => registers.borrow_mut().sub(Bits8::C, false),
            Arithmetic::SAD => registers.borrow_mut().sub(Bits8::D, false),
            Arithmetic::SAE => registers.borrow_mut().sub(Bits8::E, false),
            Arithmetic::SAH => registers.borrow_mut().sub(Bits8::H, false),
            Arithmetic::SAL => registers.borrow_mut().sub(Bits8::L, false),
            Arithmetic::SAA => registers.borrow_mut().sub(Bits8::A, false),
            Arithmetic::SAcB => registers.borrow_mut().sub(Bits8::B, true),
            Arithmetic::SAcC => registers.borrow_mut().sub(Bits8::C, true),
            Arithmetic::SAcD => registers.borrow_mut().sub(Bits8::D, true),
            Arithmetic::SAcE => registers.borrow_mut().sub(Bits8::E, true),
            Arithmetic::SAcH => registers.borrow_mut().sub(Bits8::H, true),
            Arithmetic::SAcL => registers.borrow_mut().sub(Bits8::L, true),
            Arithmetic::SAcA => registers.borrow_mut().sub(Bits8::A, true),
            Arithmetic::AAc8b => {
                Set::CalculNext(Operation::AddCarry)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::AA8b => {
                Set::CalculNext(Operation::AddNoCarry)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::AAHL => {
                Set::CalculHL(Operation::AddNoCarry)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::AAcHL => {
                Set::CalculHL(Operation::AddCarry)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::SAHL => {
                Set::CalculHL(Operation::SubNoCarry)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::SAcHL => {
                Set::CalculHL(Operation::SubCarry)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::SA8b => {
                Set::CalculNext(Operation::SubNoCarry)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::SAc8b => {
                Set::CalculNext(Operation::SubCarry)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::IncB => registers.borrow_mut().increase(Bits8::B, 1),
            Arithmetic::IncD => registers.borrow_mut().increase(Bits8::D, 1),
            Arithmetic::IncH => registers.borrow_mut().increase(Bits8::H, 1),
            Arithmetic::IncHL => {
                Set::CalculHL(Operation::Increase)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::DecB => registers.borrow_mut().decrease(Bits8::B, 1),
            Arithmetic::DecD => registers.borrow_mut().decrease(Bits8::D, 1),
            Arithmetic::DecH => registers.borrow_mut().decrease(Bits8::H, 1),
            Arithmetic::DecHL => {
                Set::CalculHL(Operation::Decrease)
                    .run(registers, memory)
                    .await?
            }
            Arithmetic::IncC => registers.borrow_mut().increase(Bits8::C, 1),
            Arithmetic::IncE => registers.borrow_mut().increase(Bits8::E, 1),
            Arithmetic::IncL => registers.borrow_mut().increase(Bits8::L, 1),
            Arithmetic::IncA => registers.borrow_mut().increase(Bits8::A, 1),
            Arithmetic::DecC => registers.borrow_mut().decrease(Bits8::C, 1),
            Arithmetic::DecE => registers.borrow_mut().decrease(Bits8::E, 1),
            Arithmetic::DecL => registers.borrow_mut().decrease(Bits8::L, 1),
            Arithmetic::DecA => registers.borrow_mut().decrease(Bits8::A, 1),
            Arithmetic::DAA => registers.borrow_mut().daa(),
            Arithmetic::SCF => registers.borrow_mut().set_carry(),
            Arithmetic::CPL => registers.borrow_mut().complement_a(),
            Arithmetic::CCF => registers.borrow_mut().complement_carry(),
        };
        Ok(cycles)
    }
}

#[cfg(test)]
mod test_arithmetic {
    use super::Arithmetic;
    use crate::registers::{Bits16, Bits8, Bus, Flag};
    use crate::{executor, Registers};
    use memory::Memory;

    #[test]
    fn test_add_next_byte_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AA8b;

        register.borrow_mut().set(Bits8::A, 0x4f);
        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        assert_eq!(register.borrow().get(Bits8::A), 0x80);
        assert!(register.borrow().get(Flag::H));
    }

    #[test]
    fn test_add_byte_at_address_hl_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAHL;

        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        assert_eq!(register.borrow().get(Bits8::A), 0xf8);
    }

    #[test]
    fn test_add_byte_in_register_b_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::AAB;

        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert!(register.borrow().get(Flag::Z));
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

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

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

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

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

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

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

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

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

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        assert_eq!(register.borrow().get(Bits8::A), 0x4e);
    }

    #[test]
    fn test_sub_byte_in_register_b_without_carry() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Arithmetic::SAB;

        register.borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert!(register.borrow().get(Flag::Z));
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

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

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

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        assert_eq!(register.borrow().get(Bits8::A), 0x4C);
    }
}
