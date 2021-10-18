use crate::registers::{futures::Jump as Async, Absolute as J, Bits16, Flag};
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::Error;

/// JP nn
/// Unconditional jump to the absolute address specified in the next 16-bits.
/// Cycle: 16
/// Bytes: 3

/// JP cc nn
/// Conditional jump to the absolute address specified by the 16-bit operand nn, depending on the condition cc.
/// Cycle: 16 / 12
/// Bytes: 3

/// JP r8
/// Unconditional jump to the relative address specified in the next 8-bits.
/// Cycle: 12

/// JP cc r8
/// Conditional jump to the relative address specified by the next 8-bits, depending on the condition cc.
/// Cycle: 12 / 8

/// Call nn
/// Unconditional function call to the absolute address specified by the 16-bit operand nn.
/// Cycle: 24

/// Call CC nn
/// Conditional function call to the absolute address specified by the 16-bit operand nn.
/// Cycle: 24 / 12

/// Flags:
///
/// Z - Unuzed
/// N - Unused
/// H - Unused
/// C - Unused

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Jump {
    NN = 0xC3,
    HL = 0xE9,
    NZNN = 0xC2,
    NCNN = 0xD2,
    ZNN = 0xCA,
    CNN = 0xDA,
    R8b = 0x18,
    NZR8b = 0x20,
    NCR8b = 0x30,
    ZR8b = 0x28,
    CR8b = 0x38,
    Call = 0xCD,
    CallZ = 0xCC,
    CallC = 0xDC,
    CallNZ = 0xC4,
    CallNC = 0xD4,
}

impl Jump {
    pub async fn exec(self, registers: Registers, memory: Memory) -> Result<(), Error> {
        match self {
            Jump::HL => registers.borrow_mut().absolute(Bits16::HL),
            Jump::Call => Async::Call.jump(registers, memory).await?,
            Jump::CallZ => Async::CallCheck(Flag::Z).jump(registers, memory).await?,
            Jump::CallC => Async::CallCheck(Flag::C).jump(registers, memory).await?,
            Jump::CallNZ => Async::CallNot(Flag::Z).jump(registers, memory).await?,
            Jump::CallNC => Async::CallNot(Flag::C).jump(registers, memory).await?,
            Jump::R8b => Async::Relative.jump(registers, memory).await?,
            Jump::NN => Async::Absolute.jump(registers, memory).await?,
            Jump::NZNN => Async::AbsoluteNot(Flag::Z).jump(registers, memory).await?,
            Jump::NCNN => Async::AbsoluteNot(Flag::C).jump(registers, memory).await?,
            Jump::NZR8b => Async::RelativeNot(Flag::Z).jump(registers, memory).await?,
            Jump::NCR8b => Async::RelativeNot(Flag::C).jump(registers, memory).await?,
            Jump::ZNN => {
                Async::AbsoluteCheck(Flag::Z)
                    .jump(registers, memory)
                    .await?
            }
            Jump::CNN => {
                Async::AbsoluteCheck(Flag::C)
                    .jump(registers, memory)
                    .await?
            }
            Jump::ZR8b => {
                Async::RelativeCheck(Flag::Z)
                    .jump(registers, memory)
                    .await?
            }
            Jump::CR8b => {
                Async::RelativeCheck(Flag::Z)
                    .jump(registers, memory)
                    .await?
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod test_jumps {
    use super::Jump;
    use crate::registers::{Bits16, Bus, Flag};
    use crate::{executor, Registers};
    use memory::Memory;

    #[test]
    fn test_jump_to_address_in_next_16bits() {
        let src = 0xc000;
        let expected = 0xc050;

        let register = Registers::default();
        let memory = Memory::default();

        let instruction = Jump::NN;
        register.borrow_mut().set(Bits16::PC, src);
        memory.borrow_mut().set_u16(0xc000, 0xc050).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jump_to_address_in_hl() {
        let src = 0xc000;
        let expected = 0xc050;

        let register = Registers::default();
        let memory = Memory::default();

        let instruction = Jump::HL;
        register.borrow_mut().set(Bits16::PC, src);
        register.borrow_mut().set(Bits16::HL, 0xc050);

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jump_to_address_in_next_16bits_if_carry_true() {
        let src = 0xc000;
        let expected = 0xc050;

        let register = Registers::default();
        let memory = Memory::default();

        let instruction = Jump::CNN;
        register.borrow_mut().set(Bits16::PC, src);
        register.borrow_mut().set(Flag::C, true);
        memory.borrow_mut().set_u16(0xc000, 0xc050).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jump_to_address_in_next_16bits_if_zero_false() {
        let src = 0xc000;
        let expected = 0xc002;

        let register = Registers::default();
        let memory = Memory::default();

        let instruction = Jump::NZNN;
        register.borrow_mut().set(Bits16::PC, src);
        register.borrow_mut().set(Flag::Z, false);
        memory.borrow_mut().set_u16(0xc000, 0xc050).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jump_to_relative_address_with_next_8bits() {
        let src: u16 = 0xc000;
        let origin: i16 = !0x18;
        let expected: u16 = 0xBFE8;

        let register = Registers::default();
        let memory = Memory::default();

        let instruction = Jump::R8b;
        register.borrow_mut().set(Bits16::PC, src);
        register.borrow_mut().set(Flag::Z, false);
        memory.borrow_mut().set_u8(0xc000, origin as u8).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result = register.borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_call() {
        let src: u16 = 0xc000;
        let dst: u16 = 0xc100;
        let stack = 0xc200;
        let expected_pc: u16 = 0xc100;
        let expected_stack: u16 = 0xc200 - 2;

        let register = Registers::default();
        let memory = Memory::default();

        let instruction = Jump::Call;
        register.borrow_mut().set(Bits16::PC, src);
        register.borrow_mut().set(Bits16::SP, stack);
        memory.borrow_mut().set_u16(0xc000, dst).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result_pc = register.borrow().get(Bits16::PC);
        let result_stack = register.borrow().get(Bits16::SP);
        assert_eq!(result_pc, expected_pc);
        assert_eq!(result_stack, expected_stack);
    }

    #[test]
    fn test_call_conditionnal_success_as_flag_c_is_true() {
        let src: u16 = 0xc000;
        let dst: u16 = 0xc100;
        let stack = 0xc200;
        let expected_pc: u16 = 0xc100;
        let expected_stack: u16 = 0xc200 - 2;

        let register = Registers::default();
        let memory = Memory::default();

        let instruction = Jump::CallC;

        register.borrow_mut().set(Bits16::PC, src);
        register.borrow_mut().set(Flag::C, true);
        register.borrow_mut().set(Bits16::SP, stack);
        memory.borrow_mut().set_u16(0xc000, dst).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result_pc = register.borrow().get(Bits16::PC);
        let result_stack = register.borrow().get(Bits16::SP);
        assert_eq!(result_pc, expected_pc);
        assert_eq!(result_stack, expected_stack);
    }

    #[test]
    fn test_call_conditionnal_success_as_flag_z_is_false() {
        let src: u16 = 0xc000;
        let dst: u16 = 0xc100;
        let stack = 0xc200;
        let expected_pc: u16 = 0xc100;
        let expected_stack: u16 = 0xc200 - 2;

        let register = Registers::default();
        let memory = Memory::default();

        let instruction = Jump::CallNZ;

        register.borrow_mut().set(Bits16::PC, src);
        register.borrow_mut().set(Bits16::SP, stack);
        memory.borrow_mut().set_u16(0xc000, dst).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result_pc = register.borrow().get(Bits16::PC);
        let result_stack = register.borrow().get(Bits16::SP);
        assert_eq!(result_pc, expected_pc);
        assert_eq!(result_stack, expected_stack);
    }

    #[test]
    fn test_call_conditionnal_failure_as_flag_z_is_true() {
        let src: u16 = 0xc000;
        let dst: u16 = 0xc100;
        let stack = 0xc200;
        let expected_pc: u16 = 0xc100;
        let expected_stack: u16 = 0xc200 - 2;

        let register = Registers::default();
        let memory = Memory::default();

        let instruction = Jump::CallNZ;

        register.borrow_mut().set(Bits16::PC, src);
        register.borrow_mut().set(Flag::Z, true);
        register.borrow_mut().set(Bits16::SP, stack);
        memory.borrow_mut().set_u16(0xc000, dst).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));

        let result_pc = register.borrow().get(Bits16::PC);
        let result_stack = register.borrow().get(Bits16::SP);
        assert_ne!(result_pc, expected_pc);
        assert_ne!(result_stack, expected_stack);
    }
}
