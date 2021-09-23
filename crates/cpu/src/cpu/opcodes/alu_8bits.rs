use super::super::area::{Bits16, Bits8, Flag};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};
use crate::cpu::flags::Flags;

///1. ADD A,n
/// Description:
///  Add n to A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),#
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Set if carry from bit 3.
///  C - Set if carry from bit 7.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  ADD A,A 87 4
///  ADD A,B 80 4
///  ADD A,C 81 4
///  ADD A,D 82 4
///  ADD A,E 83 4
///  ADD A,H 84 4
///  ADD A,L 85 4
///  ADD A,(HL) 86 8
///  ADD A,# C6 8

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum AddRegANum8bit {
    AA = 0x87,
    AB = 0x80,
    AC = 0x81,
    AD = 0x82,
    AE = 0x83,
    AH = 0x84,
    AL = 0x85,
    // AHL = 0x86,
    // AHEX = 0xc6, //A#
}

impl<'a> AddRegANum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                AddRegANum8bit::AA => Bits8::A,
                AddRegANum8bit::AB => Bits8::B,
                AddRegANum8bit::AC => Bits8::C,
                AddRegANum8bit::AD => Bits8::D,
                AddRegANum8bit::AE => Bits8::E,
                AddRegANum8bit::AH => Bits8::H,
                AddRegANum8bit::AL => Bits8::L,
                // AddRegANum8bit::AHL => Bits16::HL,
                // AddRegANum8bit::AHEX => Bits16::,
            };
            let nbr = registers.get(bits) + byte;
            registers.set(Bits8::A, nbr);

            } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 2. ADC A,n
/// Description:
///  Add n + Carry flag to A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),#
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Set if carry from bit 3.
///  C - Set if carry from bit 7.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  ADC A,A 8F 4
///  ADC A,B 88 4
///  ADC A,C 89 4
///  ADC A,D 8A 4
///  ADC A,E 8B 4
///  ADC A,H 8C 4
///  ADC A,L 8D 4
///  ADC A,(HL) 8E 8
///  ADC A,# CE 8

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum AddCRegANum8bit {
    AA = 0x8f,
    AB = 0x88,
    AC = 0x89,
    AD = 0x8a,
    AE = 0x8b,
    AH = 0x8c,
    AL = 0x8d,
    AHL = 0x8e,
    // AHEX = 0xce, //A#
}

impl<'a> AddCRegANum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                AddCRegANum8bit::AA => {
                    let nbr = registers.get(Bits8::A) + bytes + Flag::C;

                }
                AddCRegANum8bit::AB => {
                    let nbr = registers.get(Bits8::B) + bytes + Flag::C;

                }
                AddCRegANum8bit::AC => {
                    let nbr = registers.get(Bits8::C) + bytes + Flag::C;

                }
                AddCRegANum8bit::AD => {
                    let nbr = registers.get(Bits8::D) + bytes + Flag::C;

                }
                AddCRegANum8bit::AE => {
                    let nbr = registers.get(Bits8::E) + bytes + Flag::C;

                }
                AddCRegANum8bit::AH => {
                    let nbr = registers.get(Bits8::H) + bytes + Flag::C;

                }
                AddCRegANum8bit::AL => {
                    let nbr = registers.get(Bits8::L) + bytes + Flag::C;

                }
                AddCRegANum8bit::AHL => {
                    let nbr = registers.get(Bits16::HL + bytes + Flag::C;
                    registers.set(Bits8::A, nbr as u8);
                    if !nbr { Flag::Z = 1}
                }
                // AddCRegANum8bit::AHEX => {
                //     let nbr = registers.get(Bits16::) + bytes + Flag::C;
                //     registers.set(Bits8::A, nbr) },
            };
            match result {
                Ok(_) => Ok(8),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 3. SUB n
/// Description:
///  Subtract n from A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),#
/// Flags affected:
///  Z - Set if result is zero.
///  N - Set.
///  H - Set if no borrow from bit 4.
///  C - Set if no borrow.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  SUB A 97 4
///  SUB B 90 4
///  SUB C 91 4
///  SUB D 92 4
///  SUB E 93 4
///  SUB H 94 4
///  SUB L 95 4
///  SUB (HL) 96 8
///  SUB # D6 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum SubRegANum8bit {
    A = 0x97,
    B = 0x90,
    C = 0x91,
    D = 0x92,
    E = 0x93,
    H = 0x94,
    L = 0x95,
    HL = 0x96,
    // HEX = 0xd6, //#
}

impl<'a> SubRegANum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                SubRegANum8bit::A => {
                    let nbr = registers.get(Bits8::A) - bytes;

                }
                SubRegANum8bit::B => {
                    let nbr = registers.get(Bits8::B) - bytes;

                }
                SubRegANum8bit::C => {
                    let nbr = registers.get(Bits8::C) - bytes;

                }
                SubRegANum8bit::D => {
                    let nbr = registers.get(Bits8::D) - bytes;

                }
                SubRegANum8bit::E => {
                    let nbr = registers.get(Bits8::E) - bytes;

                }
                SubRegANum8bit::H => {
                    let nbr = registers.get(Bits8::H) - bytes;

                }
                SubRegANum8bit::L => {
                    let nbr = registers.get(Bits8::L) - bytes;

                }
                SubRegANum8bit::HL => {
                    let nbr = registers.get(Bits16::HL - bytes;
                    registers.set(Bits8::A, nbr as u8);
                    if !nbr { Flag::Z = 1}
                }
                // SubRegANum8bit::HEX => {
                //     let nbr = registers.get(Bits16::) - bytes;
                //     registers.set(Bits8::A, nbr) },
            };
            match result {
                Ok(_) => Ok(8),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 4. SBC A,n
/// Description:
///  Subtract n + Carry flag from A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),#
/// Flags affected:
///  Z - Set if result is zero.
///  N - Set.
///  H - Set if no borrow from bit 4.
///  C - Set if no borrow.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  SBC A,A 9F 4
///  SBC A,B 98 4
///  SBC A,C 99 4
///  SBC A,D 9A 4
///  SBC A,E 9B 4
///  SBC A,H 9C 4
///  SBC A,L 9D 4
///  SBC A,(HL) 9E 8
///  SBC A,# ?? ?

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum SubCRegANum8bit {
    AA = 0x9f,
    AB = 0x98,
    AC = 0x99,
    AD = 0x9a,
    AE = 0x9vb,
    AH = 0x9c,
    AL = 0x9d,
    AHL = 0x9e,
    // AHEX = 0x??, //A#
}

impl<'a> SubCRegANum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                SubCRegANum8bit::AA => {
                    let nbr = registers.get(Bits8::A) - (bytes + Flag::C);

                }
                SubCRegANum8bit::AB => {
                    let nbr = registers.get(Bits8::B) - (bytes + Flag::C);

                }
                SubCRegANum8bit::AC => {
                    let nbr = registers.get(Bits8::C) - (bytes + Flag::C);

                }
                SubCRegANum8bit::AD => {
                    let nbr = registers.get(Bits8::D) - (bytes + Flag::C);

                }
                SubCRegANum8bit::AE => {
                    let nbr = registers.get(Bits8::E) - (bytes + Flag::C);

                }
                SubCRegANum8bit::AH => {
                    let nbr = registers.get(Bits8::H) - (bytes + Flag::C);

                }
                SubCRegANum8bit::AL => {
                    let nbr = registers.get(Bits8::L) - (bytes + Flag::C);

                }
                SubCRegANum8bit::AHL => {
                    let nbr = registers.get(Bits16::HL - (bytes + Flag::C);
                    registers.set(Bits8::A, nbr as u8);
                    if !nbr { Flag::Z = 1}
                }
                // SubCRegANum8bit::AHEX => {
                //     let nbr = registers.get(Bits16::) - (bytes + Flag::C);
                //     registers.set(Bits8::A, nbr) },
            };
            match result {
                Ok(_) => Ok(8),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

///5. AND n
/// Description:
///  Logically AND n with A, result in A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),#
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Set.
///  C - Reset.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  AND A A7 4
///  AND B A0 4
///  AND C A1 4
///  AND D A2 4
///  AND E A3 4
///  AND H A4 4
///  AND L A5 4
///  AND (HL) A6 8
///  AND # E6 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum AndRegANum8bit {
    AA = 0xa7,
    AB = 0xa0,
    AC = 0xa1,
    AD = 0xa2,
    AE = 0xa3,
    AH = 0xa4,
    AL = 0xa5,
    AHL = 0xa6,
    // AHEX = 0xe6, //A#
}

impl<'a> AndRegANum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                AndRegANum8bit::AA => registers.get(Bits8::A) & bytes;
                AndRegANum8bit::AB => registers.get(Bits8::B) & bytes,
                AndRegANum8bit::AC => registers.get(Bits8::C) & bytes,
                AndRegANum8bit::AD => registers.get(Bits8::D) & bytes,
                AndRegANum8bit::AE => registers.get(Bits8::E) & bytes,
                AndRegANum8bit::AH => registers.get(Bits8::H) & bytes,
                AndRegANum8bit::AL => registers.get(Bits8::L) & bytes,
                AndRegANum8bit::AHL => registers.get(Bits16::HL & bytes,
                // AndRegANum8bit::AHEX => = registers.get(Bits16::) & bytes,
            };
            registers.set(Bits8::A, result)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 6. OR n
/// Description:
///  Logical OR n with register A, result in A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),#
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Reset.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  OR A B7 4
///  OR B B0 4
///  OR C B1 4
///  OR D B2 4
///  OR E B3 4
///  OR H B4 4
///  OR L B5 4
///  OR (HL) B6 8
///  OR # F6 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum OrRegANum8bit {
    AA = 0xb7,
    AB = 0xb0,
    AC = 0xb1,
    AD = 0xb2,
    AE = 0xb3,
    AH = 0xb4,
    AL = 0xb5,
    // AHL = 0xb6,
    // AHEX = 0xf6, //A#
}

impl<'a> OrRegANum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                OrRegANum8bit::AA => registers.get(Bits8::A) | bytes,
                OrRegANum8bit::AB => registers.get(Bits8::B) | bytes,
                OrRegANum8bit::AC => registers.get(Bits8::C) | bytes,
                OrRegANum8bit::AD => registers.get(Bits8::D) | bytes,
                OrRegANum8bit::AE => registers.get(Bits8::E) | bytes,
                OrRegANum8bit::AH => registers.get(Bits8::H) | bytes,
                OrRegANum8bit::AL => registers.get(Bits8::L) | bytes,
                // OrRegANum8bit::AHL => registers.get(Bits16::HL | bytes,
                // OrRegANum8bit::AHEX =>      registers.get(Bits16::) | bytes,
            };
            registers.set(Bits8::A, result)
            // registers.set_z()
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

///7. XOR n
/// Description:
///  Logical exclusive OR n with register A, result in A.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),#
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Reset.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  XOR A AF 4
///  XOR B A8 4
///  XOR C A9 4
///  XOR D AA 4
///  XOR E AB 4
///  XOR H AC 4
///  XOR L AD 4
///  XOR (HL) AE 8
///  XOR * EE 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum XorRegANum8bit {
    AA = 0xaf,
    AB = 0xa8,
    AC = 0xa9,
    AD = 0xaa,
    AE = 0xab,
    AH = 0xac,
    AL = 0xad,
    // AHL = ae,
    // AHEX = 0xee, //A#
}

impl<'a> OrRegANum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                OrRegANum8bit::AA => registers.get(Bits8::A) ^ bytes,
                OrRegANum8bit::AB => registers.get(Bits8::B) ^ bytes,
                OrRegANum8bit::AC => registers.get(Bits8::C) ^ bytes,
                OrRegANum8bit::AD => registers.get(Bits8::D) ^ bytes,
                OrRegANum8bit::AE => registers.get(Bits8::E) ^ bytes,
                OrRegANum8bit::AH => registers.get(Bits8::H) ^ bytes,
                OrRegANum8bit::AL => registers.get(Bits8::L) ^ bytes,
                // OrRegANum8bit::AHL => registers.get(Bits16::HL | bytes,
                // OrRegANum8bit::AHEX =>      registers.get(Bits16::) | bytes,
            };
            registers.set(Bits8::A, result)
            // registers.set_z()
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 8. CP n
/// Description:
///  Compare A with n. This is basically an A - n
///  subtraction instruction but the results are thrown
///  away.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL),#
/// Flags affected:
///  Z - Set if result is zero. (Set if A = n.)
///  N - Set.
///  H - Set if no borrow from bit 4.
///  C - Set for no borrow. (Set if A < n.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  CP A BF 4
///  CP B B8 4
///  CP C B9 4
///  CP D BA 4
///  CP E BB 4
///  CP H BC 4
///  CP L BD 4
///  CP (HL) BE 8
///  CP # FE 8

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum CmpRegANum8bit {
    AA = 0xaf,
    AB = 0xa8,
    AC = 0xa9,
    AD = 0xaa,
    AE = 0xab,
    AH = 0xac,
    AL = 0xad,
    // AHL = ae,
    // AHEX = 0xee, //A#
}

impl<'a> CmpRegANum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                CmpRegANum8bit::AA => registers.get(Bits8::A) - bytes,
                CmpRegANum8bit::AB => registers.get(Bits8::B) - bytes,
                CmpRegANum8bit::AC => registers.get(Bits8::C) - bytes,
                CmpRegANum8bit::AD => registers.get(Bits8::D) - bytes,
                CmpRegANum8bit::AE => registers.get(Bits8::E) - bytes,
                CmpRegANum8bit::AH => registers.get(Bits8::H) - bytes,
                CmpRegANum8bit::AL => registers.get(Bits8::L) - bytes,
                // OrRegANum8bit::AHL => registers.get(Bits16::HL | bytes,
                // OrRegANum8bit::AHEX =>      registers.get(Bits16::) | bytes,
            };
            //SET registers todo!()
            // registers.set_z()
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 9. INC n
/// Description:
///  Increment register n.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Set if carry from bit 3.
///  C - Not affected.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  INC A 3C 4
///  INC B 04 4
///  INC C 0C 4
///  INC D 14 4
///  INC E 1C 4
///  INC H 24 4
///  INC L 2C 4
///  INC (HL) 34 12

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum IncReg {
    A = 0x3c,
    B = 0x04,
    C = 0x0c,
    D = 0x14,
    E = 0x1c,
    H = 0x24,
    L = 0x2c,
    // AHL = 0x34,
}

impl<'a> IncReg {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut bits = match self {
                IncReg::A => Bits8::A,
                IncReg::B => Bits8::B,
                IncReg::C => Bits8::C,
                IncReg::D => Bits8::D,
                IncReg::E => Bits8::E,
                IncReg::H => Bits8::H,
                IncReg::L => Bits8::L,
                // IncReg::HL => Bits16::HL | bytes,
            };
            registers.set(bits, registers.get(bits) + 1)
            //SET registers F todo!()
            // registers.set_z()
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

///10. DEC n
/// Description:
///  Decrement register n.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if reselt is zero.
///  N - Set.
///  H - Set if no borrow from bit 4.
///  C - Not affected.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  DEC A 3D 4
///  DEC B 05 4
///  DEC C 0D 4
///  DEC D 15 4
///  DEC E 1D 4
///  DEC H 25 4
///  DEC L 2D 4
///  DEC (HL) 35 12

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DecReg {
    A = 0x3d,
    B = 0x05,
    C = 0x0d,
    D = 0x15,
    E = 0x1d,
    H = 0x25,
    L = 0x2d,
    // AHL = 0x35,
}

impl<'a> DecReg {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut bits = match self {
                DecReg::A => Bits8::A,
                DecReg::B => Bits8::B,
                DecReg::C => Bits8::C,
                DecReg::D => Bits8::D,
                DecReg::E => Bits8::E,
                DecReg::H => Bits8::H,
                DecReg::L => Bits8::L,
                // DecReg::HL => Bits16::HL | bytes,
            };
            registers.set(bits, registers.get(bits) - 1)
            //SET registers F todo!()
            // registers.set_z()
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}