use crate::area::Bits8;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::Flags;
use num_enum::TryFromPrimitive;

/// DAA
/// Description:
///  Decimal adjust register A.
///  This instruction adjusts register A so that the
///  correct representation of Binary Coded Decimal (BCD) is obtained.
/// Flags affected:
///  Z - Set if register A is zero.
///  N - Not affected.
///  H - Reset.
///  C - Set or reset according to operation.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// DAA         -/-        0x27   4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum DecAdjustRegA {
    DAA = 0x2f,
}

impl DecAdjustRegA {
    pub async fn exec(self, registers: Registers) {
        let mut adj = 0;

        let data = registers.borrow().get(Bits8::A);
        let mut flag: Flags = Flags::from_bytes([registers.borrow().get(Bits8::F)]);

        if flag.h() || (!flag.n() && (data & 0xf) > 9) {
            adj |= 0x6;
        }

        let c = if flag.c() || (!flag.n() && data > 0x99) {
            adj |= 0x60;
            true
        } else {
            false
        };

        let data = if flag.n() { data - adj } else { data + adj };

        registers.borrow_mut().set(Bits8::A, data);
        flag.set_z(data == 0);
        flag.set_h(false);
        flag.set_c(c);
        registers
            .borrow_mut()
            .set(Bits8::F, Flags::into_bytes(flag)[0])
    }
}

#[cfg(test)]
mod test_instruction_decimal_adjust_register_a {
    use super::DecAdjustRegA;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};

    #[test]
    fn test_instruction_daa() {
        let register = Registers::default();
        let instruction = DecAdjustRegA::DAA;
        register.borrow_mut().set(Bits8::A, 0x37);
        register.borrow_mut().set(Flag::C, true);
        executor::execute(Box::pin(instruction.exec(register.clone())));
        assert_eq!(register.borrow().get(Bits16::AF), 0x9780);
    }
}
