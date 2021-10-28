use crate::registers::{futures::Jump as Async, Absolute as J, Bits16, Flag};
use crate::{Access, Cpu};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;
use std::fmt;

use super::decode::{Decode, Decoder};

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

/// Return nn
/// Unconditional function call to the absolute address specified by the 16-bit operand nn.
/// Cycle: 16

/// Return CC nn
/// Conditional function call to the absolute address specified by the 16-bit operand nn.
/// Cycle: 20 / 8

/// Flags:
///
/// Z - Unuzed
/// N - Unused
/// H - Unused
/// C - Unused

#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
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
    Return = 0xC9,
    ReturnZ = 0xC8,
    ReturnC = 0xD8,
    ReturnNZ = 0xC0,
    ReturnNC = 0xD0,
}

impl Decoder for Jump {
    fn decode(self, cpu: Cpu) -> Decode {
        Box::pin(self.exec(cpu))
    }
}

impl Jump {
    pub async fn exec(self, cpu: Cpu) -> Result<u8, Error> {
        let cycles = match self {
            Jump::HL => cpu.registers().borrow_mut().absolute(Bits16::HL),
            Jump::Call => Async::Call.jump(cpu).await?,
            Jump::CallZ => Async::CallCheck(Flag::Z).jump(cpu).await?,
            Jump::CallC => Async::CallCheck(Flag::C).jump(cpu).await?,
            Jump::CallNZ => Async::CallNot(Flag::Z).jump(cpu).await?,
            Jump::CallNC => Async::CallNot(Flag::C).jump(cpu).await?,
            Jump::R8b => Async::Relative.jump(cpu).await?,
            Jump::NN => Async::Absolute.jump(cpu).await?,
            Jump::NZNN => Async::AbsoluteNot(Flag::Z).jump(cpu).await?,
            Jump::NCNN => Async::AbsoluteNot(Flag::C).jump(cpu).await?,
            Jump::NZR8b => Async::RelativeNot(Flag::Z).jump(cpu).await?,
            Jump::NCR8b => Async::RelativeNot(Flag::C).jump(cpu).await?,
            Jump::Return => Async::Return.jump(cpu).await?,
            Jump::ReturnZ => Async::ReturnCheck(Flag::Z).jump(cpu).await?,
            Jump::ReturnC => Async::ReturnCheck(Flag::C).jump(cpu).await?,
            Jump::ReturnNZ => Async::ReturnNot(Flag::Z).jump(cpu).await?,
            Jump::ReturnNC => Async::ReturnNot(Flag::C).jump(cpu).await?,
            Jump::ZNN => Async::AbsoluteCheck(Flag::Z).jump(cpu).await?,
            Jump::CNN => Async::AbsoluteCheck(Flag::C).jump(cpu).await?,
            Jump::ZR8b => Async::RelativeCheck(Flag::Z).jump(cpu).await?,
            Jump::CR8b => Async::RelativeCheck(Flag::Z).jump(cpu).await?,
        };
        Ok(cycles)
    }
}
impl fmt::Display for Jump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Jump::NN => write!(f, "Jump NN(16b)"),
            Jump::HL => write!(f, "Jump HL"),
            Jump::NZNN => write!(f, "Jump NN(16b) if (!Z)"),
            Jump::NCNN => write!(f, "Jump NN(16b) if (!C)"),
            Jump::ZNN => write!(f, "Jump NN(16b) if (Z)"),
            Jump::CNN => write!(f, "Jump NN(16b) if (C)"),
            Jump::R8b => write!(f, "Jump R(8b)"),
            Jump::NZR8b => write!(f, "Jump R(8b) if (!Z)"),
            Jump::NCR8b => write!(f, "Jump R(8b) if (!C)"),
            Jump::ZR8b => write!(f, "Jump R(8b) if (Z)"),
            Jump::CR8b => write!(f, "Jump R(8b) if (C)"),
            Jump::Call => write!(f, "Call"),
            Jump::CallZ => write!(f, "Call if (Z)"),
            Jump::CallC => write!(f, "Call if (C)"),
            Jump::CallNZ => write!(f, "Call if (!Z)"),
            Jump::CallNC => write!(f, "Call if (!C)"),
            Jump::Return => write!(f, "Return"),
            Jump::ReturnZ => write!(f, "Return if (Z)"),
            Jump::ReturnC => write!(f, "Return if (C)"),
            Jump::ReturnNZ => write!(f, "Return if (!Z)"),
            Jump::ReturnNC => write!(f, "Return if (!C)"),
        }
    }
}

#[cfg(test)]
mod test_jumps {
    use super::Jump;
    use crate::registers::{Bits16, Bus, Flag};
    use crate::{executor, Access, Cpu};

    #[test]
    fn test_jump_to_address_in_next_16bits() {
        let src = 0xc000;
        let expected = 0xc050;

        let cpu = Cpu::default();
        let instruction = Jump::NN;
        cpu.registers().borrow_mut().set(Bits16::PC, src);
        cpu.memory().borrow_mut().set_u16(0xc000, 0xc050).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        let result = cpu.registers().borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jump_to_address_in_hl() {
        let src = 0xc000;
        let expected = 0xc050;

        let cpu = Cpu::default();
        let instruction = Jump::HL;
        cpu.registers().borrow_mut().set(Bits16::PC, src);
        cpu.registers().borrow_mut().set(Bits16::HL, 0xc050);

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        let result = cpu.registers().borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jump_to_address_in_next_16bits_if_carry_true() {
        let src = 0xc000;
        let expected = 0xc050;

        let cpu = Cpu::default();

        let instruction = Jump::CNN;
        cpu.registers().borrow_mut().set(Bits16::PC, src);
        cpu.registers().borrow_mut().set(Flag::C, true);
        cpu.memory().borrow_mut().set_u16(0xc000, 0xc050).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        let result = cpu.registers().borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jump_to_address_in_next_16bits_if_zero_false() {
        let src = 0xc000;
        let expected = 0xc002;

        let cpu = Cpu::default();

        let instruction = Jump::NZNN;
        cpu.registers().borrow_mut().set(Bits16::PC, src);
        cpu.registers().borrow_mut().set(Flag::Z, false);
        cpu.memory().borrow_mut().set_u16(0xc000, 0xc050).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        let result = cpu.registers().borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_jump_to_relative_address_with_next_8bits() {
        let src: u16 = 0xc000;
        let origin: i16 = !0x18;
        let expected: u16 = 0xBFE8;

        let cpu = Cpu::default();

        let instruction = Jump::R8b;
        cpu.registers().borrow_mut().set(Bits16::PC, src);
        cpu.registers().borrow_mut().set(Flag::Z, false);
        cpu.memory()
            .borrow_mut()
            .set_u8(0xc000, origin as u8)
            .unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        let result = cpu.registers().borrow().get(Bits16::PC);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_call() {
        let src: u16 = 0xc000;
        let dst: u16 = 0xc100;
        let stack = 0xc200;
        let expected_pc: u16 = 0xc100;
        let expected_stack: u16 = 0xc200 - 2;

        let cpu = Cpu::default();

        let instruction = Jump::Call;
        cpu.registers().borrow_mut().set(Bits16::PC, src);
        cpu.registers().borrow_mut().set(Bits16::SP, stack);
        cpu.memory().borrow_mut().set_u16(0xc000, dst).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        let result_pc = cpu.registers().borrow().get(Bits16::PC);
        let result_stack = cpu.registers().borrow().get(Bits16::SP);
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

        let cpu = Cpu::default();

        let instruction = Jump::CallC;

        cpu.registers().borrow_mut().set(Bits16::PC, src);
        cpu.registers().borrow_mut().set(Flag::C, true);
        cpu.registers().borrow_mut().set(Bits16::SP, stack);
        cpu.memory().borrow_mut().set_u16(0xc000, dst).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        let result_pc = cpu.registers().borrow().get(Bits16::PC);
        let result_stack = cpu.registers().borrow().get(Bits16::SP);
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

        let cpu = Cpu::default();

        let instruction = Jump::CallNZ;

        cpu.registers().borrow_mut().set(Bits16::PC, src);
        cpu.registers().borrow_mut().set(Bits16::SP, stack);
        cpu.memory().borrow_mut().set_u16(0xc000, dst).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        let result_pc = cpu.registers().borrow().get(Bits16::PC);
        let result_stack = cpu.registers().borrow().get(Bits16::SP);
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

        let cpu = Cpu::default();

        let instruction = Jump::CallNZ;

        cpu.registers().borrow_mut().set(Bits16::PC, src);
        cpu.registers().borrow_mut().set(Flag::Z, true);
        cpu.registers().borrow_mut().set(Bits16::SP, stack);
        cpu.memory().borrow_mut().set_u16(0xc000, dst).unwrap();

        executor::execute(Box::pin(instruction.exec(cpu.clone())));

        let result_pc = cpu.registers().borrow().get(Bits16::PC);
        let result_stack = cpu.registers().borrow().get(Bits16::SP);
        assert_ne!(result_pc, expected_pc);
        assert_ne!(result_stack, expected_stack);
    }
}
