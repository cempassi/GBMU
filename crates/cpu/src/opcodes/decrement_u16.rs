use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use num_enum::TryFromPrimitive;

///  DEC nn
/// Description:
///  Decrement register nn.
/// Use with:
///  nn = BC,DE,HL,SP
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// DEC         BC         0x0B   8
/// DEC         DE         0x1B   8
/// DEC         HL         0x2B   8
/// DEC         SP         0x3B   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum DecRegNN {
    BC = 0x0b,
    DE = 0x1b,
    HL = 0x2b,
    SP = 0x3b,
}

impl DecRegNN {
    pub async fn exec(self, registers: Registers) {
        let dst = match self {
            DecRegNN::BC => Bits16::BC,
            DecRegNN::DE => Bits16::DE,
            DecRegNN::HL => Bits16::HL,
            DecRegNN::SP => Bits16::SP,
        };
        let data = registers.borrow().get(dst).wrapping_sub(1);
        registers.borrow_mut().set(dst, data);
    }
}

#[cfg(test)]
mod test_instruction_decrement_u16 {
    use super::DecRegNN;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};

    #[test]
    fn test_instruction_decrement_reg_sp() {
        let register = Registers::default();
        let instruction = DecRegNN::SP;
        register.borrow_mut().set(Bits16::SP, 0x42);
        executor::execute(Box::pin(instruction.exec(register.clone())));
        assert_eq!(register.borrow().get(Bits16::SP), 0x41);
    }
}
