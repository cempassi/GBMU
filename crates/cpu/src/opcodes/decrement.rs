use super::super::area::Bits8;
use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::Flags;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

const MAX_BIT3: u8 = (1 << 4) - 1;

/// DEC n
/// Description:
///  Decrement register n.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Set.
///  H - Set if no borrow from bit 4.
///  C - Not affected.
/// Opcodes:
/// Instruction Parameters Opcode Cycles  | Instruction Parameters Opcode Cycles
/// DEC         A          0x3D   4         DEC         E          0x1D   4
/// DEC         B          0x05   4         DEC         H          0x25   4
/// DEC         C          0x0D   4         DEC         L          0x2D   4
/// DEC         D          0x15   4         DEC         (HL)       0x35   12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Decrement {
    A = 0x3d,
    B = 0x05,
    C = 0x0d,
    D = 0x15,
    E = 0x1d,
    H = 0x25,
    L = 0x2d,
    HL = 0x35,
}

fn decrement_reg(dst: Bits8, register: Registers) -> Flags {
    let mut flag = Flags::default();
    let data = register.borrow().get(dst);
    flag.set_h(data & MAX_BIT3 < 1 & MAX_BIT3);
    let data = data.wrapping_sub(1);
    register.borrow_mut().set(dst, data);
    flag.set_z(data == 0);
    flag.set_n(true);
    flag
}

impl Decrement {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let flag = match self {
            Decrement::A => decrement_reg(Bits8::A, registers.clone()),
            Decrement::B => decrement_reg(Bits8::B, registers.clone()),
            Decrement::C => decrement_reg(Bits8::C, registers.clone()),
            Decrement::D => decrement_reg(Bits8::D, registers.clone()),
            Decrement::E => decrement_reg(Bits8::E, registers.clone()),
            Decrement::H => decrement_reg(Bits8::H, registers.clone()),
            Decrement::L => decrement_reg(Bits8::L, registers.clone()),
            Decrement::HL => {
                let mut flag = Flags::default();
                let dst = registers.borrow().get(Bits16::HL);
                let data = <Memory as Async>::get(memory.clone(), dst).await.unwrap();
                flag.set_h(data & MAX_BIT3 < 1 & MAX_BIT3);
                let data = data.wrapping_sub(1);
                <Memory as Async>::set(memory, dst, data).await.unwrap();
                flag.set_z(data == 0);
                flag.set_n(true);
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
mod test_instruction_decrement {
    use super::Decrement;
    use crate::area::{Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_instruction_decrement_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Decrement::B;
        register.borrow_mut().set(Bits8::B, 0x10);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::B), 0x0f);
        assert_eq!(register.borrow().get(Flag::H), true);
        assert_eq!(register.borrow().get(Flag::N), true);
    }
}
