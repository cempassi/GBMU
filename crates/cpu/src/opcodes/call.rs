use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::Cpu;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// [CALL nn] | [CALL cc,nn]
/// Description:
/// [CALL nn]       => Push address of next instruction onto stack and then jump to address nn.
/// [CALL cc,nn]    => Call address n if following condition is true:
/// Use with:
///  cc = NZ, Call if Z flag is reset.
///  cc = Z, Call if Z flag is set.
///  cc = NC, Call if C flag is reset.
///  cc = C, Call if C flag is set.
///  nn = two byte immediate value. (LS byte first.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// CALL        nn         0xCD   12
/// CALL        NZ,nn      0xC4   12
/// CALL        Z,nn       0xCC   12
/// CALL        NC,nn      0xD4   12
/// CALL        C,nn       0xDC   12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Call {
    CALL = 0xcd,
    CALLNZ = 0xc4,
    CALLZ = 0xcc,
    CALLNC = 0xd4,
    CALLC = 0xdc,
}

impl Call {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Call::CALL => (),
            Call::CALLNZ | Call::CALLZ | Call::CALLNC | Call::CALLC => if !Cpu::flags_conditions(self as u8, registers.clone()) {
                    return;
                }
        }
        let data = <Memory as Async<u16>>::get(memory.clone(), registers.borrow().get(Bits16::PC))
            .await
            .unwrap();
        Cpu::push(registers.clone(), memory.clone(), data)
            .await
            .unwrap();
        Cpu::jump(registers.clone(), data);
    }
}

#[cfg(test)]
mod test_instruction_call {
    use super::Call;
    use crate::area::Bits16;
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_call_instruction() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Call::CALL;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::PC), 0x31fe);
    }
}
