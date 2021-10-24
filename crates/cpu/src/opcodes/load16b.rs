use crate::registers::futures::Set;
use crate::registers::Bits16;
use crate::Registers;
use memory::Memory;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;

use super::decode::{Decode, Decoder};

///1. LD n,nn
/// Description:
///  Put value nn into n.
/// Use with:
///  n = BC,DE,HL,SP
///  nn = 16 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD BC,nn 01 12
///  LD DE,nn 11 12
///  LD HL,nn 21 12
///  LD SP,nn 31 12

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

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Load16b {
    PushAF = 0xf5,
    PushBC = 0xc5,
    PushDE = 0xd5,
    PushHL = 0xe5,
    PopAF = 0xf1,
    PopBC = 0xc1,
    PopDE = 0xd1,
    PopHL = 0xe1,
    BC = 0x01,
    DE = 0x11,
    HL = 0x21,
    SP = 0x31,
    A16SP = 0x08,
}

impl Decoder for Load16b {
    fn decode(self, registers: Registers, memory: Memory) -> Decode {
        Box::pin(self.exec(registers, memory))
    }
}

impl Load16b {
    pub async fn exec(self, registers: Registers, memory: Memory) -> Result<u8, Error> {
        match self {
            Load16b::PushAF => Set::Push(Bits16::AF).run(registers, memory),
            Load16b::PushBC => Set::Push(Bits16::BC).run(registers, memory),
            Load16b::PushDE => Set::Push(Bits16::DE).run(registers, memory),
            Load16b::PushHL => Set::Push(Bits16::HL).run(registers, memory),
            Load16b::PopAF => Set::Pop(Bits16::AF).run(registers, memory),
            Load16b::PopBC => Set::Pop(Bits16::BC).run(registers, memory),
            Load16b::PopDE => Set::Pop(Bits16::DE).run(registers, memory),
            Load16b::PopHL => Set::Pop(Bits16::HL).run(registers, memory),
            Load16b::BC => Set::Load16b(Bits16::BC).run(registers, memory),
            Load16b::DE => Set::Load16b(Bits16::DE).run(registers, memory),
            Load16b::HL => Set::Load16b(Bits16::HL).run(registers, memory),
            Load16b::SP => Set::Load16b(Bits16::SP).run(registers, memory),
            Load16b::A16SP => Set::Data(Bits16::SP).run(registers, memory),
        }
        .await
    }
}

#[cfg(test)]
mod test_load_register_u16 {
    use super::Load16b;
    use crate::executor;
    use crate::registers::{Bits16, Bus};
    use crate::Registers;
    use memory::Memory;

    #[test]
    fn test_load_register_bc() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load16b::BC;
        register.borrow_mut().set(Bits16::PC, 0xc000);
        memory.borrow_mut().set_u16(0xc000, 0x4242).unwrap();
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        assert_eq!(register.borrow().get(Bits16::BC), 0x4242);
    }

    #[test]
    fn test_load_to_address_at_next_u16() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load16b::A16SP;
        register.borrow_mut().set(Bits16::PC, 0xc000);
        memory.borrow_mut().set_u16(0xc000, 0xc002).unwrap();
        register.borrow_mut().set(Bits16::SP, 0x4242);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow_mut().get_u16(0xc002).unwrap();
        assert_eq!(register.borrow().get(Bits16::SP), result);
    }

    #[test]
    fn test_pop_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load16b::PopHL;
        register.borrow_mut().set(Bits16::SP, 0xc000);
        memory.borrow_mut().set_u16(0xc000, 0x4242).unwrap();
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        assert_eq!(register.borrow().get(Bits16::HL), 0x4242);
        assert_eq!(register.borrow().get(Bits16::SP), 0xc000 + 2);
    }

    #[test]
    fn test_push_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load16b::PushHL;
        register.borrow_mut().set(Bits16::SP, 0xc002);
        register.borrow_mut().set(Bits16::HL, 0x4242);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(memory.borrow().get_u16(0xc002).unwrap(), 0x4242);
        assert_eq!(register.borrow().get(Bits16::SP), 0xc000);
    }
}
