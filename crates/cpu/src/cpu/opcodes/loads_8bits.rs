use super::super::area::{Bits16, Bits8};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};

/// 1. LD nn,n
/// Description:
///  Put value nn into n.
/// Use with:
///  nn = B,C,D,E,H,L,BC,DE,HL,SP
///  n = 8 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD B,n 06 8
///  LD C,n 0E 8
///  LD D,n 16 8
///  LD E,n 1E 8
///  LD H,n 26 8
///  LD L,n 2E 8

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegNum8bit {
    B = 0x06,
    C = 0x0e,
    D = 0x16,
    E = 0x1E,
    H = 0x26,
    L = 0x2e,
}

impl<'a> LoadRegNum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let result = match self {
                LoadRegNum8bit::B => memory.set(byte.into(), registers.get(Bits8::B)),
                LoadRegNum8bit::C => memory.set(byte.into(), registers.get(Bits8::C)),
                LoadRegNum8bit::D => memory.set(byte.into(), registers.get(Bits8::D)),
                LoadRegNum8bit::E => memory.set(byte.into(), registers.get(Bits8::E)),
                LoadRegNum8bit::H => memory.set(byte.into(), registers.get(Bits8::H)),
                LoadRegNum8bit::L => memory.set(byte.into(), registers.get(Bits8::L)),
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

/// 2. LD r1,r2
/// Description:
///  Put value r2 into r1.
/// Use with:
///  r1,r2 = A,B,C,D,E,H,L,(HL)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD A,A 7F 4
///  LD A,B 78 4
///  LD A,C 79 4
///  LD A,D 7A 4
///  LD A,E 7B 4
///  LD A,H 7C 4
///  LD A,L 7D 4
///  LD A,(HL) 7E 8
///  LD B,B 40 4
///  LD B,C 41 4
///  LD B,D 42 4
///  LD B,E 43 4
///  LD B,H 44 4
///  LD B,L 45 4
///  LD B,(HL) 46 8
///  LD C,B 48 4
///  LD C,C 49 4
///  LD C,D 4A 4
///  LD C,E 4B 4
///  LD C,H 4C 4
///  LD C,L 4D 4
///  LD C,(HL) 4E 8
///  LD D,B 50 4
///  LD D,C 51 4
///  LD D,D 52 4
///  LD D,E 53 4
///  LD D,H 54 4
///  LD D,L 55 4
///  LD D,(HL) 56 8
///  LD E,B 58 4
///  LD E,C 59 4
///  LD E,D 5A 4
///  LD E,E 5B 4
///  LD E,H 5C 4
///  LD E,L 5D 4
///  LD E,(HL) 5E 8
///  LD H,B 60 4
///  LD H,C 61 4
///  LD H,D 62 4
///  LD H,E 63 4
///  LD H,H 64 4
///  LD H,L 65 4
///  LD H,(HL) 66 8
///  LD L,B 68 4
///  LD L,C 69 4
///  LD L,D 6A 4
///  LD L,E 6B 4
///  LD L,H 6C 4
///  LD L,L 6D 4
///  LD L,(HL) 6E 8
///  LD (HL),B 70 8
///  LD (HL),C 71 8
///  LD (HL),D 72 8
///  LD (HL),E 73 8
///  LD (HL),H 74 8
///  LD (HL),L 75 8
///  LD (HL),n 36 12

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadReg1Reg2 {
    AA = 0x7f,
    AB = 0x78,
    AC = 0x79,
    AD = 0x7a,
    AE = 0x7b,
    AH = 0x7c,
    AL = 0x7d,
    // AHL = 0x7e,
    BB = 0x40,
    BC = 0x41,
    BD = 0x42,
    BE = 0x43,
    BH = 0x44,
    BL = 0x45,
    // BHL = 0x46,
    CB = 0x48,
    CC = 0x49,
    CD = 0x4a,
    CE = 0x4b,
    CH = 0x4c,
    CL = 0x4d,
    // CHL = 0x4e,
    DB = 0x50,
    DC = 0x51,
    DD = 0x52,
    DE = 0x53,
    DH = 0x54,
    DL = 0x55,
    // DHL = 0x56,
    EB = 0x58,
    EC = 0x59,
    ED = 0x5A,
    EE = 0x5B,
    EH = 0x5C,
    EL = 0x5D,
    // EHL = 0x5E,
    HB = 0x60,
    HC = 0x61,
    HD = 0x62,
    HE = 0x63,
    HH = 0x64,
    HL = 0x65,
    // HHL = 0x66,
    LB = 0x68,
    LC = 0x69,
    LD = 0x6A,
    LE = 0x6B,
    LH = 0x6C,
    LL = 0x6D,
    // LHL = 0x6E,
    // HLB = 0x70,
    // HLC = 0x71,
    // HLD = 0x72,
    // HLE = 0x73,
    // HLH = 0x74,
    // HLL = 0x75,
    // HLn = 0x36,
}

impl<'a> LoadReg1Reg2 {
    pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
        match self {
            LoadReg1Reg2::AA => registers.set(Bits8::A, registers.get(Bits8::A)),
            LoadReg1Reg2::AB => registers.set(Bits8::A, registers.get(Bits8::B)),
            LoadReg1Reg2::AC => registers.set(Bits8::A, registers.get(Bits8::C)),
            LoadReg1Reg2::AD => registers.set(Bits8::A, registers.get(Bits8::D)),
            LoadReg1Reg2::AE => registers.set(Bits8::A, registers.get(Bits8::E)),
            LoadReg1Reg2::AH => registers.set(Bits8::A, registers.get(Bits8::H)),
            LoadReg1Reg2::AL => registers.set(Bits8::A, registers.get(Bits8::L)),
            LoadReg1Reg2::AHL => (), //registers.set(Bits16::A, registers.get(Bits16::HL)),
            LoadReg1Reg2::BB => registers.set(Bits8::B, registers.get(Bits8::B)),
            LoadReg1Reg2::BC => registers.set(Bits8::B, registers.get(Bits8::C)),
            LoadReg1Reg2::BD => registers.set(Bits8::B, registers.get(Bits8::D)),
            LoadReg1Reg2::BE => registers.set(Bits8::B, registers.get(Bits8::E)),
            LoadReg1Reg2::BH => registers.set(Bits8::B, registers.get(Bits8::H)),
            LoadReg1Reg2::BL => registers.set(Bits8::B, registers.get(Bits8::L)),
            LoadReg1Reg2::BHL => (), //registers.set(Bits16::B, registers.get(Bits16::HL)),
            LoadReg1Reg2::CB => registers.set(Bits8::C, registers.get(Bits8::B)),
            LoadReg1Reg2::CC => registers.set(Bits8::C, registers.get(Bits8::C)),
            LoadReg1Reg2::CD => registers.set(Bits8::C, registers.get(Bits8::D)),
            LoadReg1Reg2::CE => registers.set(Bits8::C, registers.get(Bits8::E)),
            LoadReg1Reg2::CH => registers.set(Bits8::C, registers.get(Bits8::H)),
            LoadReg1Reg2::CL => registers.set(Bits8::C, registers.get(Bits8::L)),
            LoadReg1Reg2::CHL => (), //registers.set(Bits16::C, registers.get(Bits16::HL)),
            LoadReg1Reg2::DB => registers.set(Bits8::D, registers.get(Bits8::B)),
            LoadReg1Reg2::DC => registers.set(Bits8::D, registers.get(Bits8::C)),
            LoadReg1Reg2::DD => registers.set(Bits8::D, registers.get(Bits8::D)),
            LoadReg1Reg2::DE => registers.set(Bits8::D, registers.get(Bits8::E)),
            LoadReg1Reg2::DH => registers.set(Bits8::D, registers.get(Bits8::H)),
            LoadReg1Reg2::DL => registers.set(Bits8::D, registers.get(Bits8::L)),
            LoadReg1Reg2::DHL => (), //registers.set(Bits16::D, registers.get(Bits16::HL)),
            LoadReg1Reg2::EB => registers.set(Bits8::E, registers.get(Bits8::B)),
            LoadReg1Reg2::EC => registers.set(Bits8::E, registers.get(Bits8::C)),
            LoadReg1Reg2::ED => registers.set(Bits8::E, registers.get(Bits8::D)),
            LoadReg1Reg2::EE => registers.set(Bits8::E, registers.get(Bits8::E)),
            LoadReg1Reg2::EH => registers.set(Bits8::E, registers.get(Bits8::H)),
            LoadReg1Reg2::EL => registers.set(Bits8::E, registers.get(Bits8::L)),
            LoadReg1Reg2::EHL => (), //registers.set(Bits16::E, registers.get(Bits16::HL)),
            LoadReg1Reg2::HB => registers.set(Bits8::H, registers.get(Bits8::B)),
            LoadReg1Reg2::HC => registers.set(Bits8::H, registers.get(Bits8::C)),
            LoadReg1Reg2::HD => registers.set(Bits8::H, registers.get(Bits8::D)),
            LoadReg1Reg2::HE => registers.set(Bits8::H, registers.get(Bits8::E)),
            LoadReg1Reg2::HH => registers.set(Bits8::H, registers.get(Bits8::H)),
            LoadReg1Reg2::HL => registers.set(Bits8::H, registers.get(Bits8::L)),
            LoadReg1Reg2::HHL => (), //registers.set(Bits16::C, registers.get(Bits16::HL)),
            LoadReg1Reg2::LB => registers.set(Bits8::L, registers.get(Bits8::B)),
            LoadReg1Reg2::LC => registers.set(Bits8::L, registers.get(Bits8::C)),
            LoadReg1Reg2::LD => registers.set(Bits8::L, registers.get(Bits8::D)),
            LoadReg1Reg2::LE => registers.set(Bits8::L, registers.get(Bits8::E)),
            LoadReg1Reg2::LH => registers.set(Bits8::L, registers.get(Bits8::H)),
            LoadReg1Reg2::LL => registers.set(Bits8::L, registers.get(Bits8::L)),
            LoadReg1Reg2::LHL => (), //registers.set(Bits16::L, registers.get(Bits16::HL)),
            // LoadReg1Reg2::HLB => registers.set(Bits16::HL, registers.get(Bits16::B)),
            // LoadReg1Reg2::HLC => registers.set(Bits16::HL, registers.get(Bits16::C)),
            // LoadReg1Reg2::HLD => registers.set(Bits16::HL, registers.get(Bits16::D)),
            // LoadReg1Reg2::HLE => registers.set(Bits16::HL, registers.get(Bits16::E)),
            // LoadReg1Reg2::HLH => registers.set(Bits16::HL, registers.get(Bits16::H)),
            // LoadReg1Reg2::HLL => registers.set(Bits16::HL, registers.get(Bits16::L)),
            // LoadReg1Reg2::HLn => registers.set(Bits16::HL, registers.get(Bits16::memory)),
            _ => (),
        }
        Ok(())
    }
}

/// 3. LD A,n
/// Description:
///  Put value n into A.
/// Use with:
///  n = A,B,C,D,E,H,L,(BC),(DE),(HL),(nn),#
///  nn = two byte immediate value. (LS byte first.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD A,A 7F 4
///  LD A,B 78 4
///  LD A,C 79 4
///  LD A,D 7A 4
///  LD A,E 7B 4
///  LD A,H 7C 4
///  LD A,L 7D 4
///  LD A,(BC) 0A 8
///  LD A,(DE) 1A 8
///  LD A,(HL) 7E 8
///  LD A,(nn) FA 16
///  LD A,# 3E 8

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegANum8bit {
    AA = 0x7F,
    AB = 0x78,
    AC = 0x79,
    AD = 0x7A,
    AE = 0x7B,
    AH = 0x7C,
    AL = 0x7D,
    // ABC = 0x0A,
    // ADE = 0x1A,
    // AHL = 0x7E,
    // ANN = 0xFA,
    // AHEX = 0x3E,
}

impl<'a> LoadRegANum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let res = match self {
                LoadRegANum8bit::AA => registers.set(Bits8::A, registers.get(Bits8::A)),
                LoadRegANum8bit::AB => registers.set(Bits8::A, registers.get(Bits8::B)),
                LoadRegANum8bit::AC => registers.set(Bits8::A, registers.get(Bits8::C)),
                LoadRegANum8bit::AD => registers.set(Bits8::A, registers.get(Bits8::D)),
                LoadRegANum8bit::AE => registers.set(Bits8::A, registers.get(Bits8::E)),
                LoadRegANum8bit::AH => registers.set(Bits8::A, registers.get(Bits8::H)),
                LoadRegANum8bit::AL => registers.set(Bits8::A, registers.get(Bits8::L)),
                // LoadRegANum8bit::ABC => registers.set(Bits8::A, registers.get(Bits16::BC)),
                // LoadRegANum8bit::ADE => registers.set(Bits8::A, registers.get(Bits16::De)),
                // LoadRegANum8bit::AHL => registers.set(Bits8::A, registers.get(Bits16::HL)),
                // LoadRegANum8bit::ANN => registers.set(Bits8::A, registers.get(byte.into() OU Bits16::todo!())),
                // LoadRegANum8bit::AHEX => registers.set(Bits8::A, registers.get(byte.into() OU Bits16::todo!())),
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

/// 4. LD n,A
/// Description:
///  Put value A into n.
/// Use with:
///  n = A,B,C,D,E,H,L,(BC),(DE),(HL),(nn)
///  nn = two byte immediate value. (LS byte first.)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD A,A 7F 4
///  LD B,A 47 4
///  LD C,A 4F 4
///  LD D,A 57 4
///  LD E,A 5F 4
///  LD H,A 67 4
///  LD L,A 6F 4
///  LD (BC),A 02 8
///  LD (DE),A 12 8
///  LD (HL),A 77 8
///  LD (nn),A EA 16

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadNum8bitRegA {
    AA = 0x7f,
    BA = 0x47,
    CA = 0x4f,
    DA = 0x57,
    EA = 0x5f,
    HA = 0x67,
    LA = 0x6f,
    BCA = 0x02,
    DEA = 0x12,
    HLA = 0x77,
    NNA = 0xea,
}

impl<'a> LoadNum8bitRegA {
    pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
        match self {
            LoadNum8bitRegA::AA => {}
            LoadNum8bitRegA::BA => {}
            LoadNum8bitRegA::CA => {}
            LoadNum8bitRegA::DA => {}
            LoadNum8bitRegA::EA => {}
            LoadNum8bitRegA::HA => {}
            LoadNum8bitRegA::LA => {}
            LoadNum8bitRegA::BCA => {}
            LoadNum8bitRegA::DEA => {}
            LoadNum8bitRegA::HLA => {}
            LoadNum8bitRegA::NNA => {}
        }
    }
}

/// 5. LD A,(C)
/// Description:
///  Put value at address $FF00 + register C into A.
///  Same as: LD A,($FF00+C)
///  Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD A,(C) F2 8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegARegC {
    AC = 0xf2,
}

impl<'a> LoadRegARegC {
    pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
        registers.set(Bits8::A, registers.get(0xff00 + Bits8::C))
    }
}

/// 6. LD (C),A
/// Description:
/// Put A into address $FF00 + register C.
/// Opcodes:
/// Instruction Parameters      Opcode  Cycles
/// LD          ($FF00+C),A     E2      8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegCRegA {
    CA = 0xe2,
}

impl<'a> LoadRegCRegA {
    pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
        Ok(registers.set(0xff00 + Bits8::C, registers.get(Bits8::A)))
    }
}

/// 7. LD A,(HLD)
/// Description: Same as: LDD A,(HL)
/// 8. LD A,(HL-)
/// Description: Same as: LDD A,(HL)
/// 9. LDD A,(HL)
/// Description:
///  Put value at address HL into A. Decrement HL.
///  Same as: LD A,(HL) - DEC HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD A,(HLD) 3A 8
///  LD A,(HL-) 3A 8
///  LDD A,(HL) 3A 8

// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum LoadDecRegARegHL {
//       AHLm = 0x3a,
// }
//
// impl<'a> LoadDecRegARegHL {
//     pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
//         registers.set(Bits8::A, registers.get(Bits16::HL));
//         // registers.set()
//         Ok(Bits16::HL -= 1)
//     }
// }

/// 10. LD (HLD),A
/// Description: Same as: LDD (HL),A
/// 11. LD (HL-),A
/// Description: Same as: LDD (HL),A

/// 12. LDD (HL),A
/// Description:
///  Put A into memory address HL. Decrement HL.
///  Same as: LD (HL),A - DEC HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD (HLD),A 32 8
///  LD (HL-),A 32 8
///  LDD (HL),A 32 8

// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum LoadDecRegHLRegA {
//       HLmA = 0x32,
// }
//
// impl<'a> LoadDecRegHLRegA {
//     pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
//         registers.set(Bits16::HL, registers.get(Bits8::A));
//         // registers.set()
//         Ok(Bits16::HL -= 1)
//     }
// }

/// 13. LD A,(HLI)
/// Description: Same as: LDI A,(HL)
/// 14. LD A,(HL+)
/// Description: Same as: LDI A,(HL)
/// 15. LDI A,(HL)
/// Description:
/// Put value at address HL into A. Increment HL.
///  Same as: LD A,(HL) - INC HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD A,(HLI) 2A 8
///  LD A,(HL+) 2A 8
///  LDI A,(HL) 2A 8

// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum LoadIncRegARegHL {
//       AHLp = 0x2a,
// }
//
// impl<'a> LoadIncRegARegHL {
//     pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
//         registers.set(Bits8::A, registers.get(Bits16::HL));
//         // registers.set()
//         Ok(Bits16::HL += 1)
//     }
// }

/// 16. LD (HLI),A
/// Description: Same as: LDI (HL),A
/// 17. LD (HL+),A
/// Description: Same as: LDI (HL),A
/// 18. LDI (HL),A
/// Description:
///  Put A into memory address HL. Increment HL.
///  Same as: LD (HL),A - INC HL
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD (HLI),A 22 8
///  LD (HL+),A 22 8
///  LDI (HL),A 22 8

// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum LoadIncRegHLRegA {
//       HLpA = 0x32,
// }
//
// impl<'a> LoadRegCRegA {
//     pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
//         registers.set(Bits16::HL, registers.get(Bits8::A));
//         // registers.set()
//         Ok(Bits16::HL += 1)
//     }
// }

/// 19. LDH (n),A
/// Description:
///  Put A into memory address $FF00+n.
/// Use with:
///  n = one byte immediate value.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD ($FF00+n),A E0 12

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadHNum8bitRegA {
    HnA = 0xe0,
}

impl<'a> LoadHNum8bitRegA {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            Ok(memory.set(0xff00 + byte.into(), registers.get(Bits8::A)))
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 20. LDH A,(n)
/// Description:
///  Put memory address $FF00+n into A.
/// Use with:
///  n = one byte immediate value.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD A,($FF00+n) F0 12

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegAHNum8bit {
    AHn = 0xf0,
}

impl<'a> LoadRegAHNum8bit {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            Ok(register.set(Bits8::A, memory.get(0xff00 + byte.into())))
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

// #[cfg(test)]
// mod test_loadregnum8bit {
//     use super::LoadRegNum8bit;
//     use super::memory;
//
//     #[test]
//     fn test_load_reg_B_n8() {
//         let memory = Memory::default();
//         todo!()
//     }
// }
