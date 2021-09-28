use super::super::area::Bits8;
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use num_enum::TryFromPrimitive;

/// 2. LD r1,r2
/// Description:
///  Put value r2 into r1.
/// Use with:
///  r1,r2 = A,B,C,D,E,H,L,(HL)
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// [LD A,A    7F 4]   [LD B,B    40 4]     [LD C,B    48 4]  [LD D,B    50 4]   [LD E,B    58 4] [LD H,B    60 4]  [LD L,B    68 4]  [LD (HL),B 70 8]
/// [LD A,B    78 4]   [LD B,C    41 4]     [LD C,C    49 4]  [LD D,C    51 4]   [LD E,C    59 4] [LD H,C    61 4]  [LD L,C    69 4]  [LD (HL),C 71 8]
/// [LD A,C    79 4]   [LD B,D    42 4]     [LD C,D    4A 4]  [LD D,D    52 4]   [LD E,D    5A 4] [LD H,D    62 4]  [LD L,D    6A 4]  [LD (HL),D 72 8]
/// [LD A,D    7A 4]   [LD B,E    43 4]     [LD C,E    4B 4]  [LD D,E    53 4]   [LD E,E    5B 4] [LD H,E    63 4]  [LD L,E    6B 4]  [LD (HL),E 73 8]
/// [LD A,E    7B 4]   [LD B,H    44 4]     [LD C,H    4C 4]  [LD D,H    54 4]   [LD E,H    5C 4] [LD H,H    64 4]  [LD L,H    6C 4]  [LD (HL),H 74 8]
/// [LD A,H    7C 4]   [LD B,L    45 4]     [LD C,L    4D 4]  [LD D,L    55 4]   [LD E,L    5D 4] [LD H,L    65 4]  [LD L,L    6D 4]  [LD (HL),L 75 8]
/// [LD A,L    7D 4]   [LD B,(HL) 46 8]     [LD C,(HL) 4E 8]  [LD D,(HL) 56 8]   [LD E,(HL) 5E 8] [LD H,(HL) 66 8]  [LD L,(HL) 6E 8]  [LD (HL),n 36 12]
/// [LD A,(HL) 7E 8]
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
    pub fn exec(self, registers: Registers) {
        let (r1, r2) = match self {
            LoadReg1Reg2::AA => (Bits8::A, Bits8::A),
            LoadReg1Reg2::AB => (Bits8::A, Bits8::B),
            LoadReg1Reg2::AC => (Bits8::A, Bits8::C),
            LoadReg1Reg2::AD => (Bits8::A, Bits8::D),
            LoadReg1Reg2::AE => (Bits8::A, Bits8::E),
            LoadReg1Reg2::AH => (Bits8::A, Bits8::H),
            LoadReg1Reg2::AL => (Bits8::A, Bits8::L),
            LoadReg1Reg2::BB => (Bits8::B, Bits8::B),
            LoadReg1Reg2::BC => (Bits8::B, Bits8::C),
            LoadReg1Reg2::BD => (Bits8::B, Bits8::D),
            LoadReg1Reg2::BE => (Bits8::B, Bits8::E),
            LoadReg1Reg2::BH => (Bits8::B, Bits8::H),
            LoadReg1Reg2::BL => (Bits8::B, Bits8::L),
            LoadReg1Reg2::CB => (Bits8::C, Bits8::B),
            LoadReg1Reg2::CC => (Bits8::C, Bits8::C),
            LoadReg1Reg2::CD => (Bits8::C, Bits8::D),
            LoadReg1Reg2::CE => (Bits8::C, Bits8::E),
            LoadReg1Reg2::CH => (Bits8::C, Bits8::H),
            LoadReg1Reg2::CL => (Bits8::C, Bits8::L),
            LoadReg1Reg2::DB => (Bits8::D, Bits8::B),
            LoadReg1Reg2::DC => (Bits8::D, Bits8::C),
            LoadReg1Reg2::DD => (Bits8::D, Bits8::D),
            LoadReg1Reg2::DE => (Bits8::D, Bits8::E),
            LoadReg1Reg2::DH => (Bits8::D, Bits8::H),
            LoadReg1Reg2::DL => (Bits8::D, Bits8::L),
            LoadReg1Reg2::EB => (Bits8::E, Bits8::B),
            LoadReg1Reg2::EC => (Bits8::E, Bits8::C),
            LoadReg1Reg2::ED => (Bits8::E, Bits8::D),
            LoadReg1Reg2::EE => (Bits8::E, Bits8::E),
            LoadReg1Reg2::EH => (Bits8::E, Bits8::H),
            LoadReg1Reg2::EL => (Bits8::E, Bits8::L),
            LoadReg1Reg2::HB => (Bits8::H, Bits8::B),
            LoadReg1Reg2::HC => (Bits8::H, Bits8::C),
            LoadReg1Reg2::HD => (Bits8::H, Bits8::D),
            LoadReg1Reg2::HE => (Bits8::H, Bits8::E),
            LoadReg1Reg2::HH => (Bits8::H, Bits8::H),
            LoadReg1Reg2::HL => (Bits8::H, Bits8::L),
            LoadReg1Reg2::LB => (Bits8::L, Bits8::B),
            LoadReg1Reg2::LC => (Bits8::L, Bits8::C),
            LoadReg1Reg2::LD => (Bits8::L, Bits8::D),
            LoadReg1Reg2::LE => (Bits8::L, Bits8::E),
            LoadReg1Reg2::LH => (Bits8::L, Bits8::H),
            LoadReg1Reg2::LL => (Bits8::L, Bits8::L),
        };
        registers.borrow_mut().set(r1, registers.borrow().get(r2))
    }
}
