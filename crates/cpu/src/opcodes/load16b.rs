use crate::area::Bits16;
use crate::futures::{LoadData, Pop, Push, SetData};
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

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

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
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

impl Load16b {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Load16b::PushAF => registers.push(memory, Bits16::AF),
            Load16b::PushBC => registers.push(memory, Bits16::BC),
            Load16b::PushDE => registers.push(memory, Bits16::DE),
            Load16b::PushHL => registers.push(memory, Bits16::HL),
            Load16b::PopAF => registers.pop(memory, Bits16::AF),
            Load16b::PopBC => registers.pop(memory, Bits16::BC),
            Load16b::PopDE => registers.pop(memory, Bits16::DE),
            Load16b::PopHL => registers.pop(memory, Bits16::HL),
            Load16b::BC => registers.load_data(memory, Bits16::BC),
            Load16b::DE => registers.load_data(memory, Bits16::DE),
            Load16b::HL => registers.load_data(memory, Bits16::HL),
            Load16b::SP => registers.load_data(memory, Bits16::SP),
            Load16b::A16SP => registers.set_data(memory, Bits16::SP),
        }
        .await
        .unwrap()
    }
}

#[cfg(test)]
mod test_load_register_u16 {
    use super::Load16b;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_register_bc() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load16b::BC;
        register.borrow_mut().set(Bits16::PC, 0xc000);
        memory.borrow_mut().set_u16(0xc000, 0x4242).unwrap();
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
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
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::HL), 0x4242);
        assert_eq!(register.borrow().get(Bits16::SP), 0xc000 - 2);
    }

    #[test]
    fn test_push_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load16b::PushHL;
        register.borrow_mut().set(Bits16::SP, 0xc000);
        register.borrow_mut().set(Bits16::HL, 0x4242);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(memory.borrow().get_u16(0xc000).unwrap(), 0x4242);
        assert_eq!(register.borrow().get(Bits16::SP), 0xc002);
    }
}
