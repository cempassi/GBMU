use super::decode::{Decode, Decoder};
use crate::registers::{
    futures::{Operation, Set},
    Arithmetic as A, Bits8, Complement, IncDec,
};
use crate::{Access, Cpu};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;
use std::fmt;

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
#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
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
    fn decode(self, cpu: Cpu) -> Decode {
        Box::pin(self.exec(cpu))
    }
}

impl Arithmetic {
    pub async fn exec(self, cpu: Cpu) -> Result<u8, Error> {
        let cycles = match self {
            Arithmetic::AAA => cpu.registers().borrow_mut().add(Bits8::A, false),
            Arithmetic::AAB => cpu.registers().borrow_mut().add(Bits8::B, false),
            Arithmetic::AAC => cpu.registers().borrow_mut().add(Bits8::C, false),
            Arithmetic::AAD => cpu.registers().borrow_mut().add(Bits8::D, false),
            Arithmetic::AAE => cpu.registers().borrow_mut().add(Bits8::E, false),
            Arithmetic::AAH => cpu.registers().borrow_mut().add(Bits8::H, false),
            Arithmetic::AAL => cpu.registers().borrow_mut().add(Bits8::L, false),
            Arithmetic::AAcA => cpu.registers().borrow_mut().add(Bits8::A, true),
            Arithmetic::AAcB => cpu.registers().borrow_mut().add(Bits8::B, true),
            Arithmetic::AAcC => cpu.registers().borrow_mut().add(Bits8::C, true),
            Arithmetic::AAcD => cpu.registers().borrow_mut().add(Bits8::D, true),
            Arithmetic::AAcE => cpu.registers().borrow_mut().add(Bits8::E, true),
            Arithmetic::AAcH => cpu.registers().borrow_mut().add(Bits8::H, true),
            Arithmetic::AAcL => cpu.registers().borrow_mut().add(Bits8::L, true),
            Arithmetic::SAB => cpu.registers().borrow_mut().sub(Bits8::B, false),
            Arithmetic::SAC => cpu.registers().borrow_mut().sub(Bits8::C, false),
            Arithmetic::SAD => cpu.registers().borrow_mut().sub(Bits8::D, false),
            Arithmetic::SAE => cpu.registers().borrow_mut().sub(Bits8::E, false),
            Arithmetic::SAH => cpu.registers().borrow_mut().sub(Bits8::H, false),
            Arithmetic::SAL => cpu.registers().borrow_mut().sub(Bits8::L, false),
            Arithmetic::SAA => cpu.registers().borrow_mut().sub(Bits8::A, false),
            Arithmetic::SAcB => cpu.registers().borrow_mut().sub(Bits8::B, true),
            Arithmetic::SAcC => cpu.registers().borrow_mut().sub(Bits8::C, true),
            Arithmetic::SAcD => cpu.registers().borrow_mut().sub(Bits8::D, true),
            Arithmetic::SAcE => cpu.registers().borrow_mut().sub(Bits8::E, true),
            Arithmetic::SAcH => cpu.registers().borrow_mut().sub(Bits8::H, true),
            Arithmetic::SAcL => cpu.registers().borrow_mut().sub(Bits8::L, true),
            Arithmetic::SAcA => cpu.registers().borrow_mut().sub(Bits8::A, true),
            Arithmetic::AAc8b => Set::CalculNext(Operation::AddCarry).run(cpu).await?,
            Arithmetic::AA8b => Set::CalculNext(Operation::AddNoCarry).run(cpu).await?,
            Arithmetic::AAHL => Set::CalculHL(Operation::AddNoCarry).run(cpu).await?,
            Arithmetic::AAcHL => Set::CalculHL(Operation::AddCarry).run(cpu).await?,
            Arithmetic::SAHL => Set::CalculHL(Operation::SubNoCarry).run(cpu).await?,
            Arithmetic::SAcHL => Set::CalculHL(Operation::SubCarry).run(cpu).await?,
            Arithmetic::SA8b => Set::CalculNext(Operation::SubNoCarry).run(cpu).await?,
            Arithmetic::SAc8b => Set::CalculNext(Operation::SubCarry).run(cpu).await?,
            Arithmetic::IncB => cpu.registers().borrow_mut().increase(Bits8::B, 1),
            Arithmetic::IncD => cpu.registers().borrow_mut().increase(Bits8::D, 1),
            Arithmetic::IncH => cpu.registers().borrow_mut().increase(Bits8::H, 1),
            Arithmetic::IncHL => Set::CalculHL(Operation::Increase).run(cpu).await?,
            Arithmetic::DecB => cpu.registers().borrow_mut().decrease(Bits8::B, 1),
            Arithmetic::DecD => cpu.registers().borrow_mut().decrease(Bits8::D, 1),
            Arithmetic::DecH => cpu.registers().borrow_mut().decrease(Bits8::H, 1),
            Arithmetic::DecHL => Set::CalculHL(Operation::Decrease).run(cpu).await?,
            Arithmetic::IncC => cpu.registers().borrow_mut().increase(Bits8::C, 1),
            Arithmetic::IncE => cpu.registers().borrow_mut().increase(Bits8::E, 1),
            Arithmetic::IncL => cpu.registers().borrow_mut().increase(Bits8::L, 1),
            Arithmetic::IncA => cpu.registers().borrow_mut().increase(Bits8::A, 1),
            Arithmetic::DecC => cpu.registers().borrow_mut().decrease(Bits8::C, 1),
            Arithmetic::DecE => cpu.registers().borrow_mut().decrease(Bits8::E, 1),
            Arithmetic::DecL => cpu.registers().borrow_mut().decrease(Bits8::L, 1),
            Arithmetic::DecA => cpu.registers().borrow_mut().decrease(Bits8::A, 1),
            Arithmetic::DAA => cpu.registers().borrow_mut().daa(),
            Arithmetic::SCF => cpu.registers().borrow_mut().set_carry(),
            Arithmetic::CPL => cpu.registers().borrow_mut().complement_a(),
            Arithmetic::CCF => cpu.registers().borrow_mut().complement_carry(),
        };
        Ok(cycles)
    }
}

impl fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arithmetic::AAA => write!(f, "Add A A(No Carry)"),
            Arithmetic::AAB => write!(f, "Add A B(No Carry)"),
            Arithmetic::AAC => write!(f, "Add A C(No Carry)"),
            Arithmetic::AAD => write!(f, "Add A D(No Carry)"),
            Arithmetic::AAE => write!(f, "Add A E(No Carry)"),
            Arithmetic::AAH => write!(f, "Add A H(No Carry)"),
            Arithmetic::AAL => write!(f, "Add A L(No Carry)"),
            Arithmetic::AAHL => write!(f, "Add A [HL](No Carry)"),
            Arithmetic::AA8b => write!(f, "Add A 8b(No Carry)"),
            Arithmetic::AAcA => write!(f, "Add A A(Carry)"),
            Arithmetic::AAcB => write!(f, "Add A B(Carry)"),
            Arithmetic::AAcC => write!(f, "Add A C(Carry)"),
            Arithmetic::AAcD => write!(f, "Add A D(Carry)"),
            Arithmetic::AAcE => write!(f, "Add A E(Carry)"),
            Arithmetic::AAcH => write!(f, "Add A H(Carry)"),
            Arithmetic::AAcL => write!(f, "Add A L(Carry)"),
            Arithmetic::AAcHL => write!(f, "Add A [HL](Carry)"),
            Arithmetic::AAc8b => write!(f, "Add A 8b(Carry)"),
            Arithmetic::SAB => write!(f, "Sub A B(No Carry)"),
            Arithmetic::SAC => write!(f, "Sub A C(No Carry)"),
            Arithmetic::SAD => write!(f, "Sub A D(No Carry)"),
            Arithmetic::SAE => write!(f, "Sub A E(No Carry)"),
            Arithmetic::SAH => write!(f, "Sub A H(No Carry)"),
            Arithmetic::SAL => write!(f, "Sub A L(No Carry)"),
            Arithmetic::SAHL => write!(f, "Sub A [HL](No Carry)"),
            Arithmetic::SAA => write!(f, "Sub A A(No Carry)"),
            Arithmetic::SAcB => write!(f, "Sub A B(Carry)"),
            Arithmetic::SAcC => write!(f, "Sub A C(Carry)"),
            Arithmetic::SAcD => write!(f, "Sub A D(Carry)"),
            Arithmetic::SAcE => write!(f, "Sub A E(Carry)"),
            Arithmetic::SAcH => write!(f, "Sub A H(Carry)"),
            Arithmetic::SAcL => write!(f, "Sub A L(Carry)"),
            Arithmetic::SAcHL => write!(f, "Sub A [HL](Carry)"),
            Arithmetic::SAcA => write!(f, "Sub A A (Carry)"),
            Arithmetic::SA8b => write!(f, "Sub A 8b (No Carry)"),
            Arithmetic::SAc8b => write!(f, "Sub A 8b(Carry)"),
            Arithmetic::IncB => write!(f, "Increase B"),
            Arithmetic::IncD => write!(f, "Increase D"),
            Arithmetic::IncH => write!(f, "Increase H"),
            Arithmetic::IncHL => write!(f, "Increase [HL]"),
            Arithmetic::DecB => write!(f, "Decrease B"),
            Arithmetic::DecD => write!(f, "Decrease D"),
            Arithmetic::DecH => write!(f, "Decrease H"),
            Arithmetic::DecHL => write!(f, "Decrease [HL]"),
            Arithmetic::IncC => write!(f, "Increase C"),
            Arithmetic::IncE => write!(f, "Increase E"),
            Arithmetic::IncL => write!(f, "Increase L"),
            Arithmetic::IncA => write!(f, "Increase A"),
            Arithmetic::DecC => write!(f, "Decrease C"),
            Arithmetic::DecE => write!(f, "Decrease E"),
            Arithmetic::DecL => write!(f, "Decrease L"),
            Arithmetic::DecA => write!(f, "Decrease A"),
            Arithmetic::DAA => write!(f, "Decimal Adjust (DAA)"),
            Arithmetic::SCF => write!(f, "Set Carry Flag"),
            Arithmetic::CPL => write!(f, "Complement Carry Flag"),
            Arithmetic::CCF => write!(f, "Clear Carry Flag"),
        }
    }
}

#[cfg(test)]
mod test_arithmetic {
    use super::Arithmetic;
    use crate::registers::{Bits16, Bits8, Bus, Flag};
    use crate::{executor, Access, Cpu};

    #[test]
    fn test_add_next_byte_without_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::AA8b;

        cpu.registers().borrow_mut().set(Bits8::A, 0x4f);
        cpu.registers().borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0x80);
        assert!(cpu.registers().borrow().get(Flag::H));
    }

    #[test]
    fn test_add_byte_at_address_hl_without_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::AAHL;

        cpu.registers().borrow_mut().set(Bits8::A, 0xf8);
        cpu.registers().borrow_mut().set(Bits16::HL, 0xc008);
        cpu.registers().borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(cpu.clone())));
        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0xf8);
    }

    #[test]
    fn test_add_byte_in_register_b_without_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::AAB;

        cpu.registers().borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0x00);
        assert!(cpu.registers().borrow().get(Flag::Z));
    }

    #[test]
    fn test_add_next_byte_with_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::AAc8b;

        cpu.registers().borrow_mut().set(Bits8::A, 0x4f);
        cpu.registers().borrow_mut().set(Bits16::PC, 0xc000);
        cpu.registers().borrow_mut().set(Flag::C, true);
        cpu.memory().borrow_mut().set_u8(0xc000, 0x2F).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0x7F);
    }

    #[test]
    fn test_add_byte_at_address_hl_with_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::AAcHL;

        cpu.registers().borrow_mut().set(Bits8::A, 0x2a);
        cpu.registers().borrow_mut().set(Bits16::HL, 0xc008);
        cpu.registers().borrow_mut().set(Flag::C, true);
        cpu.memory().borrow_mut().set_u8(0xc008, 0x2d).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0x58);
    }

    #[test]
    fn test_add_byte_in_register_c_with_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::AAcC;

        cpu.registers().borrow_mut().set(Bits8::A, 0x2B);
        cpu.registers().borrow_mut().set(Bits8::C, 0xAA);
        cpu.registers().borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0xD6);
    }

    #[test]
    fn test_sub_next_byte_without_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::SA8b;

        cpu.registers().borrow_mut().set(Bits8::A, 0x4f);
        cpu.registers().borrow_mut().set(Bits16::PC, 0xc000);
        cpu.registers().borrow_mut().set(Flag::C, true);
        cpu.memory().borrow_mut().set_u8(0xc000, 0x2F).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0x20);
    }

    #[test]
    fn test_sub_byte_at_address_hl_without_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::SAHL;

        cpu.registers().borrow_mut().set(Bits8::A, 0xf8);
        cpu.registers().borrow_mut().set(Flag::C, true);
        cpu.registers().borrow_mut().set(Bits16::HL, 0xc008);
        cpu.memory().borrow_mut().set_u8(0xc008, 0xaa).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0x4e);
    }

    #[test]
    fn test_sub_byte_in_register_b_without_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::SAB;

        cpu.registers().borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(cpu.clone())));
        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0x00);
        assert!(cpu.registers().borrow().get(Flag::Z));
    }

    #[test]
    fn test_sub_byte_at_address_hl_with_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::SAcHL;

        cpu.registers().borrow_mut().set(Bits8::A, 0xf8);
        cpu.registers().borrow_mut().set(Flag::C, true);
        cpu.registers().borrow_mut().set(Bits16::HL, 0xc008);
        cpu.memory().borrow_mut().set_u8(0xc008, 0xaa).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0x4d);
    }

    #[test]
    fn test_sub_byte_in_register_l_with_carry() {
        let cpu = Cpu::default();
        let instruction = Arithmetic::SAcL;

        cpu.registers().borrow_mut().set(Bits8::A, 0xF8);
        cpu.registers().borrow_mut().set(Bits8::L, 0xAB);
        cpu.registers().borrow_mut().set(Flag::C, true);

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        assert_eq!(cpu.registers().borrow().get(Bits8::A), 0x4C);
    }
}
