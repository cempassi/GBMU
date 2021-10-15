use crate::area::Bits16;
use crate::futures::{Pop, Push};
use crate::Registers;
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
pub enum Stack {
    PushAF = 0xf5,
    PushBC = 0xc5,
    PushDE = 0xd5,
    PushHL = 0xe5,
    PopAF = 0xf1,
    PopBC = 0xc1,
    PopDE = 0xd1,
    PopHL = 0xe1,
}

impl Stack {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Stack::PushAF => registers.push(memory, Bits16::AF),
            Stack::PushBC => registers.push(memory, Bits16::BC),
            Stack::PushDE => registers.push(memory, Bits16::DE),
            Stack::PushHL => registers.push(memory, Bits16::HL),
            Stack::PopAF => registers.pop(memory, Bits16::AF),
            Stack::PopBC => registers.pop(memory, Bits16::BC),
            Stack::PopDE => registers.pop(memory, Bits16::DE),
            Stack::PopHL => registers.pop(memory, Bits16::HL),
        }
        .await
        .unwrap()
    }
}

#[cfg(test)]
mod test_instruction_pop_nn {
    use super::Stack;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_pop_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Stack::PopHL;
        register.borrow_mut().set(Bits16::SP, 0xc000);
        memory.borrow_mut().set_u16(0xc000, 0x4242).unwrap();
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::HL), 0x4242);
        assert_eq!(register.borrow().get(Bits16::SP), 0xc000 - 2);
    }

    #[test]
    fn test_push_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Stack::PushHL;
        register.borrow_mut().set(Bits16::SP, 0xc000);
        register.borrow_mut().set(Bits16::HL, 0x4242);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(memory.borrow().get_u16(0xc000).unwrap(), 0x4242);
        assert_eq!(register.borrow().get(Bits16::SP), 0xc002);
    }
}
