use crate::area::Bits16;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::nextpc::NextPc;
use crate::opcodes::data::arithmetic::signed;
use crate::Cpu;
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// [JR n] | [JR cc,n]
/// Description:
///  [JR n]     => Add n to current address and jump to it.
///  [JR cc,n]  => If following condition is true then add n to current address and jump to it:
/// Use with:
///  n = one byte signed immediate value
///  cc = NZ, Jump if Z flag is reset.
///  cc = Z, Jump if Z flag is set.
///  cc = NC, Jump if C flag is reset.
///  cc = C, Jump if C flag is set.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// JR          n          0x18   8
/// JR          NZ,n       0x20   8
/// JR          Z,n        0x28   8
/// JR          NC,n       0x30   8
/// JR          C,n        0x38   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum RelJump {
    JR = 0x18,
    JRNZ = 0x20,
    JRZ = 0x28,
    JRNC = 0x30,
    JRC = 0x38,
}
impl RelJump {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            RelJump::JRNZ | RelJump::JRZ | RelJump::JRNC | RelJump::JRC => {
                if !Cpu::flags_conditions(self as u8, registers.clone()) {
                    return;
                }
            }
            RelJump::JR => (),
        };
        let src: u8 = registers.clone().next_pc(memory.clone()).await.unwrap();
        let src = signed(src) + registers.borrow().get(Bits16::PC);
        let data = <Memory as Async<u16>>::get(memory, src).await.unwrap();
        Cpu::jump(registers.clone(), data);
    }
}

#[cfg(test)]
mod test_instruction_jump {
    use super::RelJump;
    use crate::area::{Bits16, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_relative_jump() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RelJump::JR;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::PC), 0x20f3);
    }

    #[test]
    fn test_relative_jump_if_no_z() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = RelJump::JRNZ;
        register.borrow_mut().set(Flag::Z, false);
        register.borrow_mut().set(Bits16::PC, 0x4242);
        assert_eq!(register.borrow().get(Bits16::PC), 0x4242);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Bits16::PC), 0);
    }
}
