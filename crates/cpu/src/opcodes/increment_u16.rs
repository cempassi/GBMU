use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use num_enum::TryFromPrimitive;

/// INC nn
/// Description:
///  Increment register nn.
/// Use with:
///  nn = BC,DE,HL,SP
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// INC         BC         0x03   8
/// INC         DE         0x13   8
/// INC         HL         0x23   8
/// INC         SP         0x33   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum IncRegNN {
    BC = 0x03,
    DE = 0x13,
    HL = 0x23,
    SP = 0x33,
}

impl IncRegNN {
    pub async fn exec(self, registers: Registers) {
        let (dst, data) = match self {
            IncRegNN::BC => (
                Bits16::BC,
                registers.borrow().get(Bits16::BC).wrapping_add(1),
            ),
            IncRegNN::DE => (
                Bits16::DE,
                registers.borrow().get(Bits16::DE).wrapping_add(1),
            ),
            IncRegNN::HL => (
                Bits16::HL,
                registers.borrow().get(Bits16::HL).wrapping_add(1),
            ),
            IncRegNN::SP => (
                Bits16::SP,
                registers.borrow().get(Bits16::SP).wrapping_add(1),
            ),
        };
        registers.borrow_mut().set(dst, data);
    }
}

#[cfg(test)]
mod test_instruction_increment_u16 {
    use super::IncRegNN;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};

    #[test]
    fn test_instruction_increment_reg_hl() {
        let register = Registers::default();
        let instruction = IncRegNN::HL;
        register.borrow_mut().set(Bits16::HL, 0x42);
        executor::execute(Box::pin(instruction.exec(register.clone())));
        assert_eq!(register.borrow().get(Bits16::HL), 0x43);
    }
}
