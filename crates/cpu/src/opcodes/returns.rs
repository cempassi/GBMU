use super::super::area::Bits16;
use crate::cpu::Registers;
use crate::opcodes::jump::Jump;
use crate::{Cpu, RegisterBus};
use memory::Memory;
use num_enum::TryFromPrimitive;

/// [RET | RETI | RET cc]
/// Description:
/// [RET]       => Pop two bytes from stack & jump to that address.
/// [RETI]      => Pop two bytes from stack & jump to that address then enable interrupts.
/// [RET cc]    => Return if following condition is true:
/// Use with:
///     cc = NZ, Return if Z flag is reset.
///     cc = Z, Return if Z flag is set.
///     cc = NC, Return if C flag is reset.
///     cc = C, Return if C flag is set.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// RET         -/-        0xC9   8
/// RETI        -/-        0xc9   8
/// RET         NZ         0xC0   8
/// RET         Z          0xC8   8
/// RET         NC         0xD0   8
/// RET         C          0xD8   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Return {
    RETNZ = 0xc0,
    RETNC = 0xd0,
    RETZ = 0xc8,
    RETC = 0xd8,
    RETNN = 0xc9,
    RETI = 0xd9,
}

impl Return {
    pub async fn exec(self, registers: Registers, memory: Memory, interrupt: &mut bool) {
        match self {
            Return::RETNZ | Return::RETZ | Return::RETNC | Return::RETC => {
                if !Cpu::flags_conditions(self as u8, registers.clone()) {
                    return;
                }
            }
            Return::RETI => *interrupt = true,
            Return::RETNN => (),
        };
        dbg!("interrupt {:?}", interrupt);
        let data = Cpu::pop(registers.clone(), memory.clone()).await.unwrap();
        registers.borrow_mut().set(Bits16::PC, data);
        Jump::JPNN.exec(registers.clone(), memory.clone()).await;
    }
}

#[cfg(test)]
mod test_instruction_return {
    use super::Return;
    use crate::area::{Bits16, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_return_interrupt() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Return::RETI;
        let interrupt = false;
        executor::execute(Box::pin(instruction.exec(
            register.clone(),
            memory.clone(),
            &mut interrupt.clone(),
        )));
        assert_eq!(register.borrow().get(Bits16::PC), 0x31fe);
        assert_eq!(interrupt, true); // NOT WORKING
    }

    #[test]
    fn test_return_if_z() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Return::RETZ;
        register.borrow_mut().set(Flag::Z, true);
        executor::execute(Box::pin(instruction.exec(
            register.clone(),
            memory.clone(),
            &mut false,
        )));
        assert_eq!(register.borrow().get(Bits16::PC), 0x31fe);
    }
}
