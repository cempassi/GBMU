use crate::area::Bits16;
use crate::{Cpu, RegisterBus, Registers};
use memory::Memory;
use num_enum::TryFromPrimitive;

/// POP nn
/// Description:
///  Pop two bytes off stack into register pair nn.
///  Increment Stack Pointer (SP) twice.
/// Use with:
///  nn = AF,BC,DE,HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// POP         AF         0xF1   12
/// POP         BC         0xC1   12
/// POP         DE         0xD1   12
/// POP         HL         0xE1   12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Pop {
    AF = 0xf1,
    BC = 0xc1,
    DE = 0xd1,
    HL = 0xe1,
}

impl Pop {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let dst = match self {
            Pop::AF => Bits16::AF,
            Pop::BC => Bits16::BC,
            Pop::DE => Bits16::DE,
            Pop::HL => Bits16::HL,
        };
        let data = Cpu::pop(registers.clone(), memory.clone()).await.unwrap();
        dbg!("data {:?}", data);
        registers.borrow_mut().set(dst, data);
    }
}

#[cfg(test)]
mod test_instruction_pop_nn {
    use super::Pop;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_pop_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Pop::HL;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::HL), 0x31fe);
        assert_eq!(register.borrow_mut().get(Bits16::SP), 0x0002);
    }
}
