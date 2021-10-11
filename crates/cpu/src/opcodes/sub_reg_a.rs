use super::super::area::Bits8;
use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::data::arithmetic::Sub;
use crate::opcodes::data::Data;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

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
///  SUB        A,8b         0xd6 8          SBC         A,8b       ??     ?
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum SubRegA {
    AB = 0x90,
    AC = 0x91,
    AD = 0x92,
    AE = 0x93,
    AH = 0x94,
    AL = 0x95,
    AHL = 0x96,
    AA = 0x97,
    AcB = 0x98,
    AcC = 0x99,
    AcD = 0x9A,
    AcE = 0x9B,
    AcH = 0x9C,
    AcL = 0x9D,
    AcHL = 0x9E,
    AcA = 0x9F,
    A8b = 0xd6,
}

impl SubRegA {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data: Data<u8> = match self {
            SubRegA::AA => Data::NoCarry(registers.borrow().get(Bits8::A)),
            SubRegA::AB => Data::NoCarry(registers.borrow().get(Bits8::B)),
            SubRegA::AC => Data::NoCarry(registers.borrow().get(Bits8::C)),
            SubRegA::AD => Data::NoCarry(registers.borrow().get(Bits8::D)),
            SubRegA::AE => Data::NoCarry(registers.borrow().get(Bits8::E)),
            SubRegA::AH => Data::NoCarry(registers.borrow().get(Bits8::H)),
            SubRegA::AL => Data::NoCarry(registers.borrow().get(Bits8::L)),
            SubRegA::AcA => Data::Carry(registers.borrow().get(Bits8::A)),
            SubRegA::AcB => Data::Carry(registers.borrow().get(Bits8::B)),
            SubRegA::AcC => Data::Carry(registers.borrow().get(Bits8::C)),
            SubRegA::AcD => Data::Carry(registers.borrow().get(Bits8::D)),
            SubRegA::AcE => Data::Carry(registers.borrow().get(Bits8::E)),
            SubRegA::AcH => Data::Carry(registers.borrow().get(Bits8::H)),
            SubRegA::AcL => Data::Carry(registers.borrow().get(Bits8::L)),
            SubRegA::A8b => Data::NoCarry(registers.clone().next_pc(memory.clone()).await.unwrap()),
            SubRegA::AHL => {
                let src = registers.borrow().get(Bits16::HL);
                Data::NoCarry(<Memory as Async<u8>>::get(memory, src).await.unwrap())
            }
            SubRegA::AcHL => {
                let src = registers.borrow().get(Bits16::HL);
                Data::Carry(<Memory as Async<u8>>::get(memory, src).await.unwrap())
            }
        };
        let data = data.sub(registers.borrow().get(Bits8::A));
        registers.borrow_mut().set(Bits16::AF, data);
    }
}

#[cfg(test)]
mod test_instruction_sub_reg_a_8b {
    use super::SubRegA;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_sub_reg_a_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = SubRegA::A8b;
        register.borrow_mut().set(Bits8::A, 0x4f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0xe2);
        assert_eq!(register.borrow().get(Flag::H), true);
    }

    #[test]
    fn test_load_sub_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = SubRegA::AHL;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x08);
    }

    #[test]
    fn test_load_sub_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = SubRegA::AB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x00);
        assert_eq!(register.borrow().get(Flag::Z), true);
    }

    #[test]
    fn test_load_sbc_reg_a_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = SubRegA::AcHL;
        register.borrow_mut().set(Bits8::A, 0xf8);
        register.borrow_mut().set(Bits16::HL, 0xc008);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x07);
    }

    #[test]
    fn test_load_sbc_reg_a_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = SubRegA::AcB;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0xff);
    }
}
