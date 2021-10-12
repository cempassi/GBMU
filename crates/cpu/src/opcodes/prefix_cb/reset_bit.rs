use crate::area::{Bits16, Bits8};
use crate::{RegisterBus, Registers};
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// RES b,r
/// Description:
///  Reset bit b in register r.
/// Use with:
///  b = 0 - 7
///  r = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// RES         b,A        0xCB87 8
/// RES         b,B        0xCB80 8
/// RES         b,C        0xCB81 8
/// RES         b,D        0xCB82 8
/// RES         b,E        0xCB83 8
/// RES         b,H        0xCB84 8
/// RES         b,L        0xCB85 8
/// RES         b,(HL)     0xCB86 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum ResetBit {
    A0 = 0x87,
    A1 = 0x8f,
    A2 = 0x97,
    A3 = 0x9f,
    A4 = 0xa7,
    A5 = 0xaf,
    A6 = 0xb7,
    A7 = 0xbf,

    B0 = 0x80,
    B1 = 0x88,
    B2 = 0x90,
    B3 = 0x98,
    B4 = 0xa0,
    B5 = 0xa8,
    B6 = 0xb0,
    B7 = 0xb8,

    C0 = 0x81,
    C1 = 0x89,
    C2 = 0x91,
    C3 = 0x99,
    C4 = 0xa1,
    C5 = 0xa9,
    C6 = 0xb1,
    C7 = 0xb9,

    D0 = 0x82,
    D1 = 0x8a,
    D2 = 0x92,
    D3 = 0x9a,
    D4 = 0xa2,
    D5 = 0xaa,
    D6 = 0xb2,
    D7 = 0xba,

    E0 = 0x83,
    E1 = 0x8b,
    E2 = 0x93,
    E3 = 0x9b,
    E4 = 0xa3,
    E5 = 0xab,
    E6 = 0xb3,
    E7 = 0xbb,

    H0 = 0x84,
    H1 = 0x8c,
    H2 = 0x94,
    H3 = 0x9c,
    H4 = 0xa4,
    H5 = 0xac,
    H6 = 0xb4,
    H7 = 0xbc,

    L0 = 0x85,
    L1 = 0x8d,
    L2 = 0x95,
    L3 = 0x9d,
    L4 = 0xa5,
    L5 = 0xad,
    L6 = 0xb5,
    L7 = 0xbd,

    HL0 = 0x86,
    HL1 = 0x8e,
    HL2 = 0x96,
    HL3 = 0x9e,
    HL4 = 0xa6,
    HL5 = 0xae,
    HL6 = 0xb6,
    HL7 = 0xbe,
}

fn reset_bit(registers: Registers, area: Bits8, bit: u8) {
    let mut data = registers.borrow().get(area);
    data &= !(1 << bit);
    registers.borrow_mut().set(area, data);
}

async fn reset_bit_hl(registers: Registers, memory: Memory, bit: u8) {
    let address = registers.borrow().get(Bits16::HL);
    let mut data = <Memory as Async<u8>>::get(memory.clone(), address)
        .await
        .unwrap();
    data &= !(1 << bit);
    <Memory as Async<u8>>::set(memory.clone(), address, data)
        .await
        .unwrap();
}

impl ResetBit {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            ResetBit::A0 => reset_bit(registers, Bits8::A, 0),
            ResetBit::A1 => reset_bit(registers, Bits8::A, 1),
            ResetBit::A2 => reset_bit(registers, Bits8::A, 2),
            ResetBit::A3 => reset_bit(registers, Bits8::A, 3),
            ResetBit::A4 => reset_bit(registers, Bits8::A, 4),
            ResetBit::A5 => reset_bit(registers, Bits8::A, 5),
            ResetBit::A6 => reset_bit(registers, Bits8::A, 6),
            ResetBit::A7 => reset_bit(registers, Bits8::A, 7),
            ResetBit::B0 => reset_bit(registers, Bits8::B, 0),
            ResetBit::B1 => reset_bit(registers, Bits8::B, 1),
            ResetBit::B2 => reset_bit(registers, Bits8::B, 2),
            ResetBit::B3 => reset_bit(registers, Bits8::B, 3),
            ResetBit::B4 => reset_bit(registers, Bits8::B, 4),
            ResetBit::B5 => reset_bit(registers, Bits8::B, 5),
            ResetBit::B6 => reset_bit(registers, Bits8::B, 6),
            ResetBit::B7 => reset_bit(registers, Bits8::B, 7),
            ResetBit::C0 => reset_bit(registers, Bits8::C, 0),
            ResetBit::C1 => reset_bit(registers, Bits8::C, 1),
            ResetBit::C2 => reset_bit(registers, Bits8::C, 2),
            ResetBit::C3 => reset_bit(registers, Bits8::C, 3),
            ResetBit::C4 => reset_bit(registers, Bits8::C, 4),
            ResetBit::C5 => reset_bit(registers, Bits8::C, 5),
            ResetBit::C6 => reset_bit(registers, Bits8::C, 6),
            ResetBit::C7 => reset_bit(registers, Bits8::C, 7),
            ResetBit::D0 => reset_bit(registers, Bits8::D, 0),
            ResetBit::D1 => reset_bit(registers, Bits8::D, 1),
            ResetBit::D2 => reset_bit(registers, Bits8::D, 2),
            ResetBit::D3 => reset_bit(registers, Bits8::D, 3),
            ResetBit::D4 => reset_bit(registers, Bits8::D, 4),
            ResetBit::D5 => reset_bit(registers, Bits8::D, 5),
            ResetBit::D6 => reset_bit(registers, Bits8::D, 6),
            ResetBit::D7 => reset_bit(registers, Bits8::D, 7),
            ResetBit::E0 => reset_bit(registers, Bits8::E, 0),
            ResetBit::E1 => reset_bit(registers, Bits8::E, 1),
            ResetBit::E2 => reset_bit(registers, Bits8::E, 2),
            ResetBit::E3 => reset_bit(registers, Bits8::E, 3),
            ResetBit::E4 => reset_bit(registers, Bits8::E, 4),
            ResetBit::E5 => reset_bit(registers, Bits8::E, 5),
            ResetBit::E6 => reset_bit(registers, Bits8::E, 6),
            ResetBit::E7 => reset_bit(registers, Bits8::E, 7),
            ResetBit::H0 => reset_bit(registers, Bits8::H, 0),
            ResetBit::H1 => reset_bit(registers, Bits8::H, 1),
            ResetBit::H2 => reset_bit(registers, Bits8::H, 2),
            ResetBit::H3 => reset_bit(registers, Bits8::H, 3),
            ResetBit::H4 => reset_bit(registers, Bits8::H, 4),
            ResetBit::H5 => reset_bit(registers, Bits8::H, 5),
            ResetBit::H6 => reset_bit(registers, Bits8::H, 6),
            ResetBit::H7 => reset_bit(registers, Bits8::H, 7),
            ResetBit::L0 => reset_bit(registers, Bits8::L, 0),
            ResetBit::L1 => reset_bit(registers, Bits8::L, 1),
            ResetBit::L2 => reset_bit(registers, Bits8::L, 2),
            ResetBit::L3 => reset_bit(registers, Bits8::L, 3),
            ResetBit::L4 => reset_bit(registers, Bits8::L, 4),
            ResetBit::L5 => reset_bit(registers, Bits8::L, 5),
            ResetBit::L6 => reset_bit(registers, Bits8::L, 6),
            ResetBit::L7 => reset_bit(registers, Bits8::L, 7),
            ResetBit::HL0 => reset_bit_hl(registers, memory, 0).await,
            ResetBit::HL1 => reset_bit_hl(registers, memory, 1).await,
            ResetBit::HL2 => reset_bit_hl(registers, memory, 2).await,
            ResetBit::HL3 => reset_bit_hl(registers, memory, 3).await,
            ResetBit::HL4 => reset_bit_hl(registers, memory, 4).await,
            ResetBit::HL5 => reset_bit_hl(registers, memory, 5).await,
            ResetBit::HL6 => reset_bit_hl(registers, memory, 6).await,
            ResetBit::HL7 => reset_bit_hl(registers, memory, 7).await,
        }
    }
}

#[cfg(test)]
mod test_instruction_reset_bit {
    use super::ResetBit;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_instruction_reset_bit_2_reg_a() {
        let src = 0b1111_1111;
        let expected = 0b1111_1011;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = ResetBit::A2;
        register.borrow_mut().set(Bits8::A, src);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        let result = register.borrow().get(Bits8::A);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_instruction_reset_bit_7_reg_hl() {
        let hl = 0xc008;
        let src = 0b1000_0000;
        let expected = 0b0000_0000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = ResetBit::HL7;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        let result = memory.borrow().get_u8(hl).unwrap();
        assert_eq!(result, expected);
    }
}
