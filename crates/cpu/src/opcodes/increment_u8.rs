use super::super::area::Bits8;
use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::Flags;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

const MAX_BIT2: u8 = (1 << 3) - 1;

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

fn increment_reg(dst: Bits8, register: Registers) -> Flags {
    let mut flag = Flags::default();
    let data = register.borrow().get(dst);
    flag.set_h((data & MAX_BIT2) + (0x01 & MAX_BIT2) > MAX_BIT2);
    let data = data.wrapping_add(1);
    register.borrow_mut().set(dst, data);
    flag.set_z(data == 0);
    flag.set_n(false);
    flag
}

impl Increment {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let flag = match self {
            Increment::A => increment_reg(Bits8::A, registers.clone()),
            Increment::B => increment_reg(Bits8::B, registers.clone()),
            Increment::C => increment_reg(Bits8::C, registers.clone()),
            Increment::D => increment_reg(Bits8::D, registers.clone()),
            Increment::E => increment_reg(Bits8::E, registers.clone()),
            Increment::H => increment_reg(Bits8::H, registers.clone()),
            Increment::L => increment_reg(Bits8::L, registers.clone()),
            Increment::HL => {
                let mut flag = Flags::default();
                let dst = registers.borrow().get(Bits16::HL);
                let data = <Memory as Async>::get(memory.clone(), dst).await.unwrap();
                flag.set_h((data & MAX_BIT2) + (0x01 & MAX_BIT2) > MAX_BIT2);
                let data = data.wrapping_add(1);
                <Memory as Async>::set(memory, dst, data).await.unwrap();
                flag.set_z(data == 0);
                flag.set_n(false);
                flag
            }
        };
        let old_flag = registers.borrow().get(Bits8::F);
        registers
            .borrow_mut()
            .set(Bits8::F, old_flag | Flags::into_bytes(flag)[0]); //flag 'not affected' C-a-d OU avec le nouveau flag et l'ancien ..
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
