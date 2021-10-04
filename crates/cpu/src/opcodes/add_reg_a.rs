use super::super::area::Bits8;
use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::arithmetics::Data;
use crate::opcodes::Add;
use crate::Flags;
use memory::{Async, Memory};
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
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum AddRegA {
    AA = 0x8f,
    AB = 0x88,
    AC = 0x89,
    AD = 0x8a,
    AE = 0x8b,
    AH = 0x8c,
    AL = 0x8d,
    AHL = 0x8e,
    A8b = 0xce,
    AcA = 0x87,
    AcB = 0x80,
    AcC = 0x81,
    AcD = 0x82,
    AcE = 0x83,
    AcH = 0x84,
    AcL = 0x85,
    AcHL = 0x86,
    Ac8b = 0xc6,
}

impl AddRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data: Data<u8> = match self {
            AddRegA::AA => Data::NoCarry(registers.borrow().get(Bits8::A)),
            AddRegA::AB => Data::NoCarry(registers.borrow().get(Bits8::B)),
            AddRegA::AC => Data::NoCarry(registers.borrow().get(Bits8::C)),
            AddRegA::AD => Data::NoCarry(registers.borrow().get(Bits8::D)),
            AddRegA::AE => Data::NoCarry(registers.borrow().get(Bits8::E)),
            AddRegA::AH => Data::NoCarry(registers.borrow().get(Bits8::H)),
            AddRegA::AL => Data::NoCarry(registers.borrow().get(Bits8::L)),
            AddRegA::AcA => Data::Carry(registers.borrow().get(Bits8::A)),
            AddRegA::AcB => Data::Carry(registers.borrow().get(Bits8::B)),
            AddRegA::AcC => Data::Carry(registers.borrow().get(Bits8::C)),
            AddRegA::AcD => Data::Carry(registers.borrow().get(Bits8::D)),
            AddRegA::AcE => Data::Carry(registers.borrow().get(Bits8::E)),
            AddRegA::AcH => Data::Carry(registers.borrow().get(Bits8::H)),
            AddRegA::AcL => Data::Carry(registers.borrow().get(Bits8::L)),
            AddRegA::Ac8b => Data::Carry(registers.clone().next_pc(memory.clone()).await.unwrap()),
            AddRegA::A8b => Data::NoCarry(registers.clone().next_pc(memory.clone()).await.unwrap()),
            AddRegA::AHL => {
                let src = registers.borrow().get(Bits16::HL);
                Data::NoCarry(<Memory as Async>::get(memory, src).await.unwrap())
            }
            AddRegA::AcHL => {
                let src = registers.borrow().get(Bits16::HL);
                Data::Carry(<Memory as Async>::get(memory, src).await.unwrap())
            }
        };
        let (data, flag) = data.add(registers.borrow().get(Bits8::A));
        let data = (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16;
        registers.borrow_mut().set(Bits16::AF, data);
    }
}

#[cfg(test)]
mod test_instruction_add_reg_a_8b {
    use super::AddRegA;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_add_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegA::A8b;
        register.borrow_mut().set(Bits8::A, 0x4f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x80);
        assert_eq!(register.borrow().get(Flag::H), true);
    }

    #[test]
    fn test_load_add_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegA::AHL;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0xf8);
    }

    #[test]
    fn test_load_add_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegA::AB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_load_adc_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegA::Ac8b;
        register.borrow_mut().set(Bits8::A, 0x4f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x81);
        assert_eq!(register.borrow().get(Flag::H), true);
    }

    #[test]
    fn test_load_adc_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegA::AcHL;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0xf9);
    }

    #[test]
    fn test_load_adc_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = AddRegA::AcB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x01);
    }
}
