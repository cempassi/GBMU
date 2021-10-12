use crate::area::{Bits16, Bits8, Flag};
use crate::{RegisterBus, Registers};
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// BIT b,r
/// Description:
///  Test bit b in register r.
/// Use with:
///  b = 0 - 7
///  r = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if bit b of register r is 0.
///  N - Reset.
///  H - Set.
///  C - Not affected.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// BIT         b,A        0xCB47 8
/// BIT         b,B        0xCB40 8
/// BIT         b,C        0xCB41 8
/// BIT         b,D        0xCB42 8
/// BIT         b,E        0xCB43 8
/// BIT         b,H        0xCB44 8
/// BIT         b,L        0xCB45 8
/// BIT         b,(HL)     0xCB46 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum TestBit {
    A0 = 0x47,
    A1 = 0x4f,
    A2 = 0x57,
    A3 = 0x5f,
    A4 = 0x67,
    A5 = 0x6f,
    A6 = 0x77,
    A7 = 0x7f,

    B0 = 0x40,
    B1 = 0x48,
    B2 = 0x50,
    B3 = 0x58,
    B4 = 0x60,
    B5 = 0x68,
    B6 = 0x70,
    B7 = 0x78,

    C0 = 0x41,
    C1 = 0x49,
    C2 = 0x51,
    C3 = 0x59,
    C4 = 0x61,
    C5 = 0x69,
    C6 = 0x71,
    C7 = 0x79,

    D0 = 0x42,
    D1 = 0x4a,
    D2 = 0x52,
    D3 = 0x5a,
    D4 = 0x62,
    D5 = 0x6a,
    D6 = 0x72,
    D7 = 0x7a,

    E0 = 0x43,
    E1 = 0x4b,
    E2 = 0x53,
    E3 = 0x5b,
    E4 = 0x63,
    E5 = 0x6b,
    E6 = 0x73,
    E7 = 0x7b,

    H0 = 0x44,
    H1 = 0x4c,
    H2 = 0x54,
    H3 = 0x5c,
    H4 = 0x64,
    H5 = 0x6c,
    H6 = 0x74,
    H7 = 0x7c,

    L0 = 0x45,
    L1 = 0x4d,
    L2 = 0x55,
    L3 = 0x5d,
    L4 = 0x65,
    L5 = 0x6d,
    L6 = 0x75,
    L7 = 0x7d,

    HL0 = 0x46,
    HL1 = 0x4e,
    HL2 = 0x56,
    HL3 = 0x5e,
    HL4 = 0x66,
    HL5 = 0x6e,
    HL6 = 0x76,
    HL7 = 0x7e,
}

fn test_bit(registers: Registers, area: Bits8, bit: u8) {
    let data = registers.borrow().get(area);
    let z = data & (1 << bit) == 0;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().set(Flag::H, true);
    registers.borrow_mut().set(Flag::N, false);
    registers.borrow_mut().set(Flag::Z, z);
}

async fn test_bit_hl(registers: Registers, memory: Memory, bit: u8) {
    let address = registers.borrow().get(Bits16::HL);
    let data = <Memory as Async<u8>>::get(memory.clone(), address)
        .await
        .unwrap();
    let z = data & (1 << bit) == 0;
    registers.borrow_mut().set(Flag::H, true);
    registers.borrow_mut().set(Flag::N, false);
    registers.borrow_mut().set(Flag::Z, z);
}

impl TestBit {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            TestBit::A0 => test_bit(registers, Bits8::A, 0),
            TestBit::A1 => test_bit(registers, Bits8::A, 1),
            TestBit::A2 => test_bit(registers, Bits8::A, 2),
            TestBit::A3 => test_bit(registers, Bits8::A, 3),
            TestBit::A4 => test_bit(registers, Bits8::A, 4),
            TestBit::A5 => test_bit(registers, Bits8::A, 5),
            TestBit::A6 => test_bit(registers, Bits8::A, 6),
            TestBit::A7 => test_bit(registers, Bits8::A, 7),
            TestBit::B0 => test_bit(registers, Bits8::B, 0),
            TestBit::B1 => test_bit(registers, Bits8::B, 1),
            TestBit::B2 => test_bit(registers, Bits8::B, 2),
            TestBit::B3 => test_bit(registers, Bits8::B, 3),
            TestBit::B4 => test_bit(registers, Bits8::B, 4),
            TestBit::B5 => test_bit(registers, Bits8::B, 5),
            TestBit::B6 => test_bit(registers, Bits8::B, 6),
            TestBit::B7 => test_bit(registers, Bits8::B, 7),
            TestBit::C0 => test_bit(registers, Bits8::C, 0),
            TestBit::C1 => test_bit(registers, Bits8::C, 1),
            TestBit::C2 => test_bit(registers, Bits8::C, 2),
            TestBit::C3 => test_bit(registers, Bits8::C, 3),
            TestBit::C4 => test_bit(registers, Bits8::C, 4),
            TestBit::C5 => test_bit(registers, Bits8::C, 5),
            TestBit::C6 => test_bit(registers, Bits8::C, 6),
            TestBit::C7 => test_bit(registers, Bits8::C, 7),
            TestBit::D0 => test_bit(registers, Bits8::D, 0),
            TestBit::D1 => test_bit(registers, Bits8::D, 1),
            TestBit::D2 => test_bit(registers, Bits8::D, 2),
            TestBit::D3 => test_bit(registers, Bits8::D, 3),
            TestBit::D4 => test_bit(registers, Bits8::D, 4),
            TestBit::D5 => test_bit(registers, Bits8::D, 5),
            TestBit::D6 => test_bit(registers, Bits8::D, 6),
            TestBit::D7 => test_bit(registers, Bits8::D, 7),
            TestBit::E0 => test_bit(registers, Bits8::E, 0),
            TestBit::E1 => test_bit(registers, Bits8::E, 1),
            TestBit::E2 => test_bit(registers, Bits8::E, 2),
            TestBit::E3 => test_bit(registers, Bits8::E, 3),
            TestBit::E4 => test_bit(registers, Bits8::E, 4),
            TestBit::E5 => test_bit(registers, Bits8::E, 5),
            TestBit::E6 => test_bit(registers, Bits8::E, 6),
            TestBit::E7 => test_bit(registers, Bits8::E, 7),
            TestBit::H0 => test_bit(registers, Bits8::H, 0),
            TestBit::H1 => test_bit(registers, Bits8::H, 1),
            TestBit::H2 => test_bit(registers, Bits8::H, 2),
            TestBit::H3 => test_bit(registers, Bits8::H, 3),
            TestBit::H4 => test_bit(registers, Bits8::H, 4),
            TestBit::H5 => test_bit(registers, Bits8::H, 5),
            TestBit::H6 => test_bit(registers, Bits8::H, 6),
            TestBit::H7 => test_bit(registers, Bits8::H, 7),
            TestBit::L0 => test_bit(registers, Bits8::L, 0),
            TestBit::L1 => test_bit(registers, Bits8::L, 1),
            TestBit::L2 => test_bit(registers, Bits8::L, 2),
            TestBit::L3 => test_bit(registers, Bits8::L, 3),
            TestBit::L4 => test_bit(registers, Bits8::L, 4),
            TestBit::L5 => test_bit(registers, Bits8::L, 5),
            TestBit::L6 => test_bit(registers, Bits8::L, 6),
            TestBit::L7 => test_bit(registers, Bits8::L, 7),
            TestBit::HL0 => test_bit_hl(registers, memory, 0).await,
            TestBit::HL1 => test_bit_hl(registers, memory, 1).await,
            TestBit::HL2 => test_bit_hl(registers, memory, 2).await,
            TestBit::HL3 => test_bit_hl(registers, memory, 3).await,
            TestBit::HL4 => test_bit_hl(registers, memory, 4).await,
            TestBit::HL5 => test_bit_hl(registers, memory, 5).await,
            TestBit::HL6 => test_bit_hl(registers, memory, 6).await,
            TestBit::HL7 => test_bit_hl(registers, memory, 7).await,
        }
    }
}

#[cfg(test)]
mod test_instruction_test_bit {
    use super::TestBit;
    use crate::area::{Bits16, Bits8, Flag};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_instruction_test_bit() {
        let src = 0b0000_0100;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = TestBit::A2;
        register.borrow_mut().set(Bits8::A, src);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        assert_eq!(register.borrow().get(Flag::Z), false);
    }

    #[test]
    fn test_shift_memory_hl_right_logically() {
        let hl = 0xc008;
        let src = 0b0000_0000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = TestBit::HL7;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(register.borrow().get(Flag::Z), true);
    }
}
