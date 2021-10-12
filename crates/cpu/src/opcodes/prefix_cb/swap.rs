use crate::area::{Bits16, Bits8, Flag};
use crate::{RegisterBus, Registers};
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// SWAP n
/// Description:
///  Swap upper & lower nibbles of n.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Reset.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// SWAP        A          0xCB37 8
/// SWAP        B          0xCB30 8
/// SWAP        C          0xCB31 8
/// SWAP        D          0xCB32 8
/// SWAP        E          0xCB33 8
/// SWAP        H          0xCB34 8
/// SWAP        L          0xCB35 8
/// SWAP        (HL)       0xCB36 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Swap {
    A = 0x37,
    B = 0x30,
    C = 0x31,
    D = 0x32,
    E = 0x33,
    H = 0x34,
    L = 0x35,
    HL = 0x36,
}

fn swap(registers: Registers, area: Bits8) {
    let mut data = registers.borrow().get(area);
    data = (data & 0x0F) << 4 | (data & 0xF0) >> 4;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().set(Flag::C, false);
    registers.borrow_mut().set(Flag::H, false);
    registers.borrow_mut().set(Flag::N, false);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    }
}

async fn swap_hl(registers: Registers, memory: Memory) {
    let address = registers.borrow().get(Bits16::HL);
    let mut data = <Memory as Async<u8>>::get(memory.clone(), address)
        .await
        .unwrap();
    data = (data & 0x0F) << 4 | (data & 0xF0) >> 4;
    <Memory as Async<u8>>::set(memory.clone(), address, data)
        .await
        .unwrap();
    registers.borrow_mut().set(Flag::C, false);
    registers.borrow_mut().set(Flag::H, false);
    registers.borrow_mut().set(Flag::N, false);
    if data == 0 {
        registers.borrow_mut().set(Flag::Z, true);
    }
}

impl Swap {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Swap::A => swap(registers, Bits8::A),
            Swap::B => swap(registers, Bits8::B),
            Swap::C => swap(registers, Bits8::C),
            Swap::D => swap(registers, Bits8::D),
            Swap::E => swap(registers, Bits8::E),
            Swap::H => swap(registers, Bits8::H),
            Swap::L => swap(registers, Bits8::L),
            Swap::HL => swap_hl(registers, memory).await,
        }
    }
}

#[cfg(test)]
mod test_instruction_swap {
    use super::Swap;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_swap_register() {
        let src = 0b1100_0011;
        let expected = 0b0011_1100;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Swap::A;
        register.borrow_mut().set(Bits8::A, src);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        let result = register.borrow().get(Bits8::A);
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_swap_hl() {
        let hl = 0xc008;
        let src = 0b1111_0000;
        let expected = 0b0000_1111;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Swap::HL;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();

        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));

        let result = memory.borrow().get_u8(hl).unwrap();
        println!("result  : {:#b}", result);
        println!("expected: {:#b}", expected);
        assert_eq!(result, expected);
    }
}
