use super::super::area::Bits8;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::Flags;
use num_enum::TryFromPrimitive;

///CPL
/// Description:
///  Complement A register. (Flip all bits.)
/// Flags affected:
///  Z - Not affected.
///  N - Set.
///  H - Set.
///  C - Not affected.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  CPL -/- 2F 4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum CPLRegA {
    CPL = 0x2f,
}

impl CPLRegA {
    pub async fn exec(self, registers: Registers) {
        let mut data = registers.borrow().get(Bits8::A);
        data ^= 0xff;
        let mut flag: Flags = Flags::from_bytes([registers.borrow().get(Bits8::F)]);
        flag.set_h(true);
        flag.set_n(true);
        registers.borrow_mut().set(Bits8::A, data);
        registers
            .borrow_mut()
            .set(Bits8::F, Flags::into_bytes(flag)[0]);
    }
}

#[cfg(test)]
mod test_instruction_complement {
    use super::CPLRegA;
    use crate::area::{Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};

    #[test]
    fn test_instruction_complement_register_a() {
        let register = Registers::default();
        let instruction = CPLRegA::CPL;
        register.borrow_mut().set(Bits8::A, 0xf0);
        register.borrow_mut().set(Flag::Z, true);
        executor::execute(Box::pin(instruction.exec(register.clone())));
        assert_eq!(register.borrow().get(Bits8::A), 0x0f);
        assert_eq!(register.borrow().get(Flag::Z), true);
        assert_eq!(register.borrow().get(Flag::N), true);
    }
}
