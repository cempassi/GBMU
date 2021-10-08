use super::super::area::Bits16;
use crate::cpu::Registers;
use crate::opcodes::Call;
use crate::RegisterBus;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// RST n
/// Description:
///  Push present address onto stack.
///  Jump to address $0000 + n.
/// Use with:
///  n = $00,$08,$10,$18,$20,$28,$30,$38
/// Opcodes:
/// Instruction Parameters Opcode Cycles | Instruction Parameters Opcode Cycles
/// RST         00H        0xC7   32       RST         00H        0xC7  32
/// RST         08H        0xCF   32       RST         08H        0xCF  32
/// RST         10H        0xD7   32       RST         10H        0xD7  32
/// RST         18H        0xDF   32       RST         18H        0xDF  32
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Restart {
    H00 = 0xc7,
    H08 = 0xcf,
    H10 = 0xd7,
    H18 = 0xdf,
    H20 = 0xe7,
    H28 = 0xef,
    H30 = 0xf7,
    H38 = 0xff,
}

impl Restart {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let data = match self {
            Restart::H00 => 0x0000,
            Restart::H08 => 0x0008,
            Restart::H10 => 0x0010,
            Restart::H18 => 0x0018,
            Restart::H20 => 0x0020,
            Restart::H28 => 0x0028,
            Restart::H30 => 0x0030,
            Restart::H38 => 0x0038,
        };
        registers.borrow_mut().set(Bits16::PC, data); // DO not use Cpu::push(), cause a restart is a CALL with NN not a push NN + CALL
        Call::CALL.exec(registers.clone(), memory.clone()).await;
    }
}

#[cfg(test)]
mod test_instruction_restart {
    use super::Restart;
    use crate::{executor, Registers};
    use memory::Memory;

    #[test]
    fn test_restart_instruction() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Restart::H38;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(memory.borrow().get_u16(0x0038).unwrap(), 0x081a);
    }
}
