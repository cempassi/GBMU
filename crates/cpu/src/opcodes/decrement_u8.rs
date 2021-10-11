use super::super::area::Bits8;
use crate::area::{Bits16, Flag};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::opcodes::consts::BIT3_MINUS_1;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

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
pub enum DecRegN {
    A = 0x3d,
    B = 0x05,
    C = 0x0d,
    D = 0x15,
    E = 0x1d,
    H = 0x25,
    L = 0x2d,
    HL = 0x35,
}

impl DecRegN {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = if self == DecRegN::HL {
            let src = registers.borrow().get(Bits16::HL);
            let data = <Memory as Async<u8>>::get(memory.clone(), src)
                .await
                .unwrap();
            <Memory as Async<u8>>::set(memory.clone(), src, data.wrapping_sub(1))
                .await
                .unwrap();
            data
        } else {
            let src = match self {
                DecRegN::A => Bits8::A,
                DecRegN::B => Bits8::B,
                DecRegN::C => Bits8::C,
                DecRegN::D => Bits8::D,
                DecRegN::E => Bits8::E,
                DecRegN::H => Bits8::H,
                DecRegN::L => Bits8::L,
                DecRegN::HL => unreachable!(),
            };
            let data = registers.borrow_mut().get(src);
            registers.borrow_mut().set(src, data.wrapping_sub(1));
            data
        };
        registers.borrow_mut().set(Flag::Z, data == 0);
        registers.borrow_mut().set(
            Flag::H,
            (data & BIT3_MINUS_1 as u8) < (1 & BIT3_MINUS_1 as u8),
        );
        registers.borrow_mut().set(Flag::N, true);
    }
}

#[cfg(test)]
mod test_instruction_decrement {
    use super::DecRegN;
    use crate::area::{Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_instruction_decrement_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = DecRegN::B;
        register.borrow_mut().set(Bits8::B, 0x10);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits8::B), 0x0f);
        assert_eq!(register.borrow().get(Flag::H), true);
        assert_eq!(register.borrow().get(Flag::N), true);
    }
}
