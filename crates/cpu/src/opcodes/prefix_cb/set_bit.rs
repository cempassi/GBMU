use crate::area::{Bits16, Bits8};
use crate::{RegisterBus, Registers};
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// SET b,r
/// Description:
///  Set bit b in register r.
/// Use with:
///  b = 0 - 7
///  r = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// SET         b,A        0xCBC7 8
/// SET         b,B        0xCBC0 8
/// SET         b,C        0xCBC1 8
/// SET         b,D        0xCBC2 8
/// SET         b,E        0xCBC3 8
/// SET         b,H        0xCBC4 8
/// SET         b,L        0xCBC5 8
/// SET         b,(HL)     0xCBC6 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum SetBit {
    A0 = 0xc7,
    A1 = 0xcf,
    A2 = 0xd7,
    A3 = 0xdf,
    A4 = 0xe7,
    A5 = 0xef,
    A6 = 0xf7,
    A7 = 0xff,

    B0 = 0xc0,
    B1 = 0xc8,
    B2 = 0xd0,
    B3 = 0xd8,
    B4 = 0xe0,
    B5 = 0xe8,
    B6 = 0xf0,
    B7 = 0xf8,

    C0 = 0xc1,
    C1 = 0xc9,
    C2 = 0xd1,
    C3 = 0xd9,
    C4 = 0xe1,
    C5 = 0xe9,
    C6 = 0xf1,
    C7 = 0xf9,

    D0 = 0xc2,
    D1 = 0xca,
    D2 = 0xd2,
    D3 = 0xda,
    D4 = 0xe2,
    D5 = 0xea,
    D6 = 0xf2,
    D7 = 0xfa,

    E0 = 0xc3,
    E1 = 0xcb,
    E2 = 0xd3,
    E3 = 0xdb,
    E4 = 0xe3,
    E5 = 0xeb,
    E6 = 0xf3,
    E7 = 0xfb,

    H0 = 0xc4,
    H1 = 0xcc,
    H2 = 0xd4,
    H3 = 0xdc,
    H4 = 0xe4,
    H5 = 0xec,
    H6 = 0xf4,
    H7 = 0xfc,

    L0 = 0xc5,
    L1 = 0xcd,
    L2 = 0xd5,
    L3 = 0xdd,
    L4 = 0xe5,
    L5 = 0xed,
    L6 = 0xf5,
    L7 = 0xfd,

    HL0 = 0xc6,
    HL1 = 0xce,
    HL2 = 0xd6,
    HL3 = 0xde,
    HL4 = 0xe6,
    HL5 = 0xee,
    HL6 = 0xf6,
    HL7 = 0xfe,
}

fn set_bit(registers: Registers, area: Bits8, bit: u8) {
    let mut data = registers.borrow().get(area);
    data |= 1 << bit;
    registers.borrow_mut().set(area, data);
}

async fn set_bit_hl(registers: Registers, memory: Memory, bit: u8) {
    let address = registers.borrow().get(Bits16::HL);
    let mut data = <Memory as Async<u8>>::get(memory.clone(), address)
        .await
        .unwrap();
    data |= 1 << bit;
    <Memory as Async<u8>>::set(memory.clone(), address, data)
        .await
        .unwrap();
}

impl SetBit {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            SetBit::A0 => set_bit(registers, Bits8::A, 0),
            SetBit::A1 => set_bit(registers, Bits8::A, 1),
            SetBit::A2 => set_bit(registers, Bits8::A, 2),
            SetBit::A3 => set_bit(registers, Bits8::A, 3),
            SetBit::A4 => set_bit(registers, Bits8::A, 4),
            SetBit::A5 => set_bit(registers, Bits8::A, 5),
            SetBit::A6 => set_bit(registers, Bits8::A, 6),
            SetBit::A7 => set_bit(registers, Bits8::A, 7),
            SetBit::B0 => set_bit(registers, Bits8::B, 0),
            SetBit::B1 => set_bit(registers, Bits8::B, 1),
            SetBit::B2 => set_bit(registers, Bits8::B, 2),
            SetBit::B3 => set_bit(registers, Bits8::B, 3),
            SetBit::B4 => set_bit(registers, Bits8::B, 4),
            SetBit::B5 => set_bit(registers, Bits8::B, 5),
            SetBit::B6 => set_bit(registers, Bits8::B, 6),
            SetBit::B7 => set_bit(registers, Bits8::B, 7),
            SetBit::C0 => set_bit(registers, Bits8::C, 0),
            SetBit::C1 => set_bit(registers, Bits8::C, 1),
            SetBit::C2 => set_bit(registers, Bits8::C, 2),
            SetBit::C3 => set_bit(registers, Bits8::C, 3),
            SetBit::C4 => set_bit(registers, Bits8::C, 4),
            SetBit::C5 => set_bit(registers, Bits8::C, 5),
            SetBit::C6 => set_bit(registers, Bits8::C, 6),
            SetBit::C7 => set_bit(registers, Bits8::C, 7),
            SetBit::D0 => set_bit(registers, Bits8::D, 0),
            SetBit::D1 => set_bit(registers, Bits8::D, 1),
            SetBit::D2 => set_bit(registers, Bits8::D, 2),
            SetBit::D3 => set_bit(registers, Bits8::D, 3),
            SetBit::D4 => set_bit(registers, Bits8::D, 4),
            SetBit::D5 => set_bit(registers, Bits8::D, 5),
            SetBit::D6 => set_bit(registers, Bits8::D, 6),
            SetBit::D7 => set_bit(registers, Bits8::D, 7),
            SetBit::E0 => set_bit(registers, Bits8::E, 0),
            SetBit::E1 => set_bit(registers, Bits8::E, 1),
            SetBit::E2 => set_bit(registers, Bits8::E, 2),
            SetBit::E3 => set_bit(registers, Bits8::E, 3),
            SetBit::E4 => set_bit(registers, Bits8::E, 4),
            SetBit::E5 => set_bit(registers, Bits8::E, 5),
            SetBit::E6 => set_bit(registers, Bits8::E, 6),
            SetBit::E7 => set_bit(registers, Bits8::E, 7),
            SetBit::H0 => set_bit(registers, Bits8::H, 0),
            SetBit::H1 => set_bit(registers, Bits8::H, 1),
            SetBit::H2 => set_bit(registers, Bits8::H, 2),
            SetBit::H3 => set_bit(registers, Bits8::H, 3),
            SetBit::H4 => set_bit(registers, Bits8::H, 4),
            SetBit::H5 => set_bit(registers, Bits8::H, 5),
            SetBit::H6 => set_bit(registers, Bits8::H, 6),
            SetBit::H7 => set_bit(registers, Bits8::H, 7),
            SetBit::L0 => set_bit(registers, Bits8::L, 0),
            SetBit::L1 => set_bit(registers, Bits8::L, 1),
            SetBit::L2 => set_bit(registers, Bits8::L, 2),
            SetBit::L3 => set_bit(registers, Bits8::L, 3),
            SetBit::L4 => set_bit(registers, Bits8::L, 4),
            SetBit::L5 => set_bit(registers, Bits8::L, 5),
            SetBit::L6 => set_bit(registers, Bits8::L, 6),
            SetBit::L7 => set_bit(registers, Bits8::L, 7),
            SetBit::HL0 => set_bit_hl(registers, memory, 0).await,
            SetBit::HL1 => set_bit_hl(registers, memory, 1).await,
            SetBit::HL2 => set_bit_hl(registers, memory, 2).await,
            SetBit::HL3 => set_bit_hl(registers, memory, 3).await,
            SetBit::HL4 => set_bit_hl(registers, memory, 4).await,
            SetBit::HL5 => set_bit_hl(registers, memory, 5).await,
            SetBit::HL6 => set_bit_hl(registers, memory, 6).await,
            SetBit::HL7 => set_bit_hl(registers, memory, 7).await,
        }
    }
}

#[cfg(test)]
mod test_instruction_set_bit {
    use super::SetBit;
    use crate::area::{Bits16, Bits8};
    use crate::{executor, RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_instruction_set_bit_2_reg_a() {
        let src = 0b1111_1011;
        let expected = 0b1111_1111;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = SetBit::A2;
        register.borrow_mut().set(Bits8::A, src);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        let result = register.borrow().get(Bits8::A);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_instruction_set_bit_7_reg_hl() {
        let hl = 0xc008;
        let src = 0b0000_0000;
        let expected = 0b1000_0000;
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = SetBit::HL7;
        register.borrow_mut().set(Bits16::HL, hl);
        memory.borrow_mut().set_u8(hl, src).unwrap();
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        let result = memory.borrow().get_u8(hl).unwrap();
        assert_eq!(result, expected);
    }
}
