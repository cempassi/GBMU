use crate::registers::{futures::Jump as Action, Absolute as J, Bits16, Flag};
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

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
}

impl Jump {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Jump::R8b => Action::Relative.jump(memory, registers).await.unwrap(),
            Jump::NN => Action::Absolute.jump(memory, registers).await.unwrap(),
            Jump::HL => registers.borrow_mut().absolute(Bits16::HL),
            Jump::ZNN => Action::AbsoluteCheck(Flag::Z)
                .jump(memory, registers)
                .await
                .unwrap(),
            Jump::CNN => Action::AbsoluteCheck(Flag::C)
                .jump(memory, registers)
                .await
                .unwrap(),
            Jump::NZNN => Action::AbsoluteNot(Flag::Z)
                .jump(memory, registers)
                .await
                .unwrap(),
            Jump::NCNN => Action::AbsoluteNot(Flag::C)
                .jump(memory, registers)
                .await
                .unwrap(),
            Jump::NZR8b => Action::RelativeNot(Flag::Z)
                .jump(memory, registers)
                .await
                .unwrap(),
            Jump::NCR8b => Action::RelativeNot(Flag::C)
                .jump(memory, registers)
                .await
                .unwrap(),
            Jump::ZR8b => Action::RelativeCheck(Flag::Z)
                .jump(memory, registers)
                .await
                .unwrap(),
            Jump::CR8b => Action::RelativeCheck(Flag::Z)
                .jump(memory, registers)
                .await
                .unwrap(),
        };
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
}
