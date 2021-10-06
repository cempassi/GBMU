use super::super::area::Bits8;
use crate::area::{Bits16, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::opcodes::data::consts::MAX_BIT3;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// INC n
/// Description:
///  Increment register n.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Set if carry from bit 3.
///  C - Not affected.
/// Opcodes:
/// Instruction Parameters Opcode Cycles  | Instruction Parameters Opcode Cycles
/// INC         A          0x3C   4         INC         E          0x1C   4
/// INC         B          0x04   4         INC         H          0x24   4
/// INC         C          0x0C   4         INC         L          0x2C   4
/// INC         D          0x14   4         INC         (HL)       0x34   12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Increment {
    A = 0x3c,
    B = 0x04,
    C = 0x0c,
    D = 0x14,
    E = 0x1c,
    H = 0x24,
    L = 0x2c,
    HL = 0x34,
}

impl Increment {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = if self == Increment::HL {
            let src = registers.borrow().get(Bits16::HL);
            let data = <Memory as Async>::get(memory.clone(), src).await.unwrap();
            <Memory as Async>::set(memory.clone(), src, data.wrapping_add(1))
                .await
                .unwrap();
            data
        } else {
            let src = match self {
                Increment::A => Bits8::A,
                Increment::B => Bits8::B,
                Increment::C => Bits8::C,
                Increment::D => Bits8::D,
                Increment::E => Bits8::E,
                Increment::H => Bits8::H,
                Increment::L => Bits8::L,
                Increment::HL => unreachable!(),
            };
            let data = registers.borrow_mut().get(src);
            registers.borrow_mut().set(src, data.wrapping_add(1));
            data
        };
        registers.borrow_mut().set(Flag::Z, data == 0);
        registers.borrow_mut().set(
            Flag::H,
            (data & MAX_BIT3 as u8) + (1 & MAX_BIT3 as u8) > MAX_BIT3 as u8,
        );
        registers.borrow_mut().set(Flag::N, false);
    }
}

#[cfg(test)]
mod test_instruction_increment {
    use super::Increment;
    use crate::area::{Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_instruction_increment_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Increment::B;
        register.borrow_mut().set(Bits8::B, 0x0f);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::B), 0x10);
        assert_eq!(register.borrow().get(Flag::H), true);
        assert_eq!(register.borrow().get(Flag::N), false);
    }
}
