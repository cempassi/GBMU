use crate::area::Bits16;
use crate::{Cpu, RegisterBus, Registers};
use memory::Memory;
use num_enum::TryFromPrimitive;

/// PUSH nn
/// Description:
///  Push register pair nn onto stack.
///  Decrement Stack Pointer (SP) twice.
/// Use with:
///  nn = AF,BC,DE,HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  PUSH AF F5 16
///  PUSH BC C5 16
///  PUSH DE D5 16
///  PUSH HL E5 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Push {
    AF = 0xf5,
    BC = 0xc5,
    DE = 0xd5,
    HL = 0xe5,
}

impl Push {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let src = match self {
            Push::AF => Bits16::AF,
            Push::BC => Bits16::BC,
            Push::DE => Bits16::DE,
            Push::HL => Bits16::HL,
        };
        let data = registers.borrow().get(src);
        Cpu::push(registers.clone(), memory.clone(), data)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod test_instruction_push_nn {
    use super::Push;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_push_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Push::HL;
        register.borrow_mut().set(Bits16::SP, 0xc002);
        register.borrow_mut().set(Bits16::HL, 0x4242);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(memory.borrow_mut().get_u16(0xc002).unwrap(), 0x4242);
        assert_eq!(register.borrow_mut().get(Bits16::SP), 0xc000);
    }
}
