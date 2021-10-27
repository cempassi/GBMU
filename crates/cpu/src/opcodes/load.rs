use crate::cpu::Registers;
use crate::registers::futures::Set;
use crate::registers::{Bits16, Bits8, Load as L};
use memory::Memory;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;

use super::decode::{Decode, Decoder};

/// 2. LD r1,r2
/// Description:
///  Put value r2 into r1.
/// Use with:
///  r1,r2 = A,B,C,D,E,H,L
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// [LD A,A    7F 4]   [LD B,B    40 4]     [LD C,B    48 4]  [LD D,B    50 4]   [LD E,B    58 4] [LD H,B    60 4]  [LD L,B    68 4]
/// [LD A,B    78 4]   [LD B,C    41 4]     [LD C,C    49 4]  [LD D,C    51 4]   [LD E,C    59 4] [LD H,C    61 4]  [LD L,C    69 4]
/// [LD A,C    79 4]   [LD B,D    42 4]     [LD C,D    4A 4]  [LD D,D    52 4]   [LD E,D    5A 4] [LD H,D    62 4]  [LD L,D    6A 4]
/// [LD A,D    7A 4]   [LD B,E    43 4]     [LD C,E    4B 4]  [LD D,E    53 4]   [LD E,E    5B 4] [LD H,E    63 4]  [LD L,E    6B 4]
/// [LD A,E    7B 4]   [LD B,H    44 4]     [LD C,H    4C 4]  [LD D,H    54 4]   [LD E,H    5C 4] [LD H,H    64 4]  [LD L,H    6C 4]
/// [LD A,H    7C 4]   [LD B,L    45 4]     [LD C,L    4D 4]  [LD D,L    55 4]   [LD E,L    5D 4] [LD H,L    65 4]  [LD L,L    6D 4]
/// [LD A,L    7D 4]

/// 1. LD nn,n
/// Description:
///  Put value n into nn.
/// Use with:
///  nn = B,C,D,E,H,L,A
///  n = 8 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          B,n        0x06   8
/// LD          C,n        0x0e   8
/// LD          D,n        0x16   8
/// LD          E,n        0x1e   8
/// LD          H,n        0x26   8
/// LD          L,n        0x2e   8
/// LD          A,n        0x3e   8

/// LD n, (HL)
/// Description:
///  Put value (HL) into n.
/// Use with:
///  n = B,C,D,E,H,L,A
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          B,(HL)     0x46   8
/// LD          C,(HL)     0x4E   8
/// LD          D,(HL)     0x56   8
/// LD          E,(HL)     0x5E   8
/// LD          H,(HL)     0x66   8
/// LD          L,(HL)     0x6E   8
/// LD          A,(HL)     0x7E   8

/// LD (HL), n
/// Description:
///  Put value n into (HL).
/// Use with:
///  n = B,C,D,E,H,L,A
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  LD          (HL),B     0x70     8
///  LD          (HL),C     0x71     8
///  LD          (HL),D     0x72     8
///  LD          (HL),E     0x73     8
///  LD          (HL),H     0x74     8
///  LD          (HL),L     0x75     8
///  LD          (HL),A     0x77     8

/// 1. LD HL,n
/// Description:
///  Put value n into HL.
/// Use with:
///  n = 8 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          (HL),n     0x36     12

/// 1. LD [r16], A
/// Description:
/// Store value in register A into byte pointed to by register r16.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          (BC), A     0x02     8
/// LD          (DE), A     0x12     8

/// 1. LD A, [r16]
/// Description:
/// Store value in byte pointed to by register r16 in register A .
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          A, (BC)     0x0A     8
/// LD          A, (DE)     0x1A     8

/// 1. LD (HL+/-), A
/// Description:
/// Store value in register A into byte pointed to by register HL, then (increase/decrease) HL.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          (HL+), A     0x22     8
/// LD          (HL-), A     0x32     8

/// 1. LD A, (HL+/-)
/// Description:
/// Store value in byte pointed to by register (HL) in register A, then (increase/decrease) HL.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          A, (HL+)      0x2A     8
/// LD          A, (HL-)      0x3A     8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Load {
    B8b = 0x06,
    C8b = 0x0e,
    D8b = 0x16,
    E8b = 0x1e,
    H8b = 0x26,
    L8b = 0x2e,
    A8b = 0x3e,
    AA = 0x7f,
    AB = 0x78,
    AC = 0x79,
    AD = 0x7a,
    AE = 0x7b,
    AH = 0x7c,
    AL = 0x7d,
    BB = 0x40,
    BC = 0x41,
    BD = 0x42,
    BE = 0x43,
    BH = 0x44,
    BL = 0x45,
    CB = 0x48,
    CC = 0x49,
    CD = 0x4a,
    CE = 0x4b,
    CH = 0x4c,
    CL = 0x4d,
    DB = 0x50,
    DC = 0x51,
    DD = 0x52,
    DE = 0x53,
    DH = 0x54,
    DL = 0x55,
    EB = 0x58,
    EC = 0x59,
    ED = 0x5A,
    EE = 0x5B,
    EH = 0x5C,
    EL = 0x5D,
    HB = 0x60,
    HC = 0x61,
    HD = 0x62,
    HE = 0x63,
    HH = 0x64,
    HL = 0x65,
    LB = 0x68,
    LC = 0x69,
    LD = 0x6A,
    LE = 0x6B,
    LH = 0x6C,
    LL = 0x6D,
    BHL = 0x46,
    CHL = 0x4E,
    DHL = 0x56,
    EHL = 0x5E,
    HHL = 0x66,
    LHL = 0x6E,
    AHL = 0x7E,
    HLB = 0x70,
    HLC = 0x71,
    HLD = 0x72,
    HLE = 0x73,
    HLH = 0x74,
    HLL = 0x75,
    HLA = 0x77,
    HL8b = 0x36,
    BCA = 0x02,
    DEA = 0x12,
    ABC = 0x0A,
    ADE = 0x1A,
    HLPA = 0x22,
    HLMA = 0x32,
    AHLP = 0x2A,
    AHLM = 0x3A,
    ToIOC = 0xE2,
    IOC = 0xF2,
    ToIONext = 0xE0,
    IONext = 0xF0,
}

impl Decoder for Load {
    fn decode(self, registers: Registers, memory: Memory) -> Decode {
        Box::pin(self.exec(registers, memory))
    }
}

impl Load {
    pub async fn exec(self, registers: Registers, memory: Memory) -> Result<u8, Error> {
        let cycles = match self {
            Load::AA => registers.borrow_mut().load(Bits8::A, Bits8::A),
            Load::AB => registers.borrow_mut().load(Bits8::A, Bits8::B),
            Load::AC => registers.borrow_mut().load(Bits8::A, Bits8::C),
            Load::AD => registers.borrow_mut().load(Bits8::A, Bits8::D),
            Load::AE => registers.borrow_mut().load(Bits8::A, Bits8::E),
            Load::AH => registers.borrow_mut().load(Bits8::A, Bits8::H),
            Load::AL => registers.borrow_mut().load(Bits8::A, Bits8::L),
            Load::BB => registers.borrow_mut().load(Bits8::B, Bits8::B),
            Load::BC => registers.borrow_mut().load(Bits8::B, Bits8::C),
            Load::BD => registers.borrow_mut().load(Bits8::B, Bits8::D),
            Load::BE => registers.borrow_mut().load(Bits8::B, Bits8::E),
            Load::BH => registers.borrow_mut().load(Bits8::B, Bits8::H),
            Load::BL => registers.borrow_mut().load(Bits8::B, Bits8::L),
            Load::CB => registers.borrow_mut().load(Bits8::C, Bits8::B),
            Load::CC => registers.borrow_mut().load(Bits8::C, Bits8::C),
            Load::CD => registers.borrow_mut().load(Bits8::C, Bits8::D),
            Load::CE => registers.borrow_mut().load(Bits8::C, Bits8::E),
            Load::CH => registers.borrow_mut().load(Bits8::C, Bits8::H),
            Load::CL => registers.borrow_mut().load(Bits8::C, Bits8::L),
            Load::DB => registers.borrow_mut().load(Bits8::D, Bits8::B),
            Load::DC => registers.borrow_mut().load(Bits8::D, Bits8::C),
            Load::DD => registers.borrow_mut().load(Bits8::D, Bits8::D),
            Load::DE => registers.borrow_mut().load(Bits8::D, Bits8::E),
            Load::DH => registers.borrow_mut().load(Bits8::D, Bits8::H),
            Load::DL => registers.borrow_mut().load(Bits8::D, Bits8::L),
            Load::EB => registers.borrow_mut().load(Bits8::E, Bits8::B),
            Load::EC => registers.borrow_mut().load(Bits8::E, Bits8::C),
            Load::ED => registers.borrow_mut().load(Bits8::E, Bits8::D),
            Load::EE => registers.borrow_mut().load(Bits8::E, Bits8::E),
            Load::EH => registers.borrow_mut().load(Bits8::E, Bits8::H),
            Load::EL => registers.borrow_mut().load(Bits8::E, Bits8::L),
            Load::HB => registers.borrow_mut().load(Bits8::H, Bits8::B),
            Load::HC => registers.borrow_mut().load(Bits8::H, Bits8::C),
            Load::HD => registers.borrow_mut().load(Bits8::H, Bits8::D),
            Load::HE => registers.borrow_mut().load(Bits8::H, Bits8::E),
            Load::HH => registers.borrow_mut().load(Bits8::H, Bits8::H),
            Load::HL => registers.borrow_mut().load(Bits8::H, Bits8::L),
            Load::LB => registers.borrow_mut().load(Bits8::L, Bits8::B),
            Load::LC => registers.borrow_mut().load(Bits8::L, Bits8::C),
            Load::LD => registers.borrow_mut().load(Bits8::L, Bits8::D),
            Load::LE => registers.borrow_mut().load(Bits8::L, Bits8::E),
            Load::LH => registers.borrow_mut().load(Bits8::L, Bits8::H),
            Load::LL => registers.borrow_mut().load(Bits8::L, Bits8::L),
            Load::HLB => Set::HL(Bits8::B).run(registers, memory).await?,
            Load::HLC => Set::HL(Bits8::C).run(registers, memory).await?,
            Load::HLD => Set::HL(Bits8::D).run(registers, memory).await?,
            Load::HLE => Set::HL(Bits8::E).run(registers, memory).await?,
            Load::HLH => Set::HL(Bits8::H).run(registers, memory).await?,
            Load::HLL => Set::HL(Bits8::L).run(registers, memory).await?,
            Load::HLA => Set::HL(Bits8::A).run(registers, memory).await?,
            Load::HLPA => Set::Increase.run(registers, memory).await?,
            Load::HLMA => Set::Decrease.run(registers, memory).await?,
            Load::AHLP => Set::LoadIncrease.run(registers, memory).await?,
            Load::AHLM => Set::LoadDecrease.run(registers, memory).await?,
            Load::HL8b => Set::LoadHL8b.run(registers, memory).await?,
            Load::B8b => Set::Load8b(Bits8::B).run(registers, memory).await?,
            Load::C8b => Set::Load8b(Bits8::C).run(registers, memory).await?,
            Load::D8b => Set::Load8b(Bits8::D).run(registers, memory).await?,
            Load::E8b => Set::Load8b(Bits8::E).run(registers, memory).await?,
            Load::H8b => Set::Load8b(Bits8::H).run(registers, memory).await?,
            Load::L8b => Set::Load8b(Bits8::L).run(registers, memory).await?,
            Load::A8b => Set::Load8b(Bits8::A).run(registers, memory).await?,
            Load::BHL => Set::LoadHL(Bits8::B).run(registers, memory).await?,
            Load::CHL => Set::LoadHL(Bits8::C).run(registers, memory).await?,
            Load::DHL => Set::LoadHL(Bits8::D).run(registers, memory).await?,
            Load::EHL => Set::LoadHL(Bits8::E).run(registers, memory).await?,
            Load::HHL => Set::LoadHL(Bits8::H).run(registers, memory).await?,
            Load::LHL => Set::LoadHL(Bits8::L).run(registers, memory).await?,
            Load::AHL => Set::LoadHL(Bits8::A).run(registers, memory).await?,
            Load::BCA => {
                Set::RegisterAt(Bits16::BC, Bits8::A)
                    .run(registers, memory)
                    .await?
            }
            Load::DEA => {
                Set::RegisterAt(Bits16::DE, Bits8::A)
                    .run(registers, memory)
                    .await?
            }
            Load::ABC => {
                Set::LoadRegisterFrom(Bits8::A, Bits16::DE)
                    .run(registers, memory)
                    .await?
            }
            Load::ADE => {
                Set::LoadRegisterFrom(Bits8::A, Bits16::DE)
                    .run(registers, memory)
                    .await?
            }
            Load::ToIOC => Set::IOC.run(registers, memory).await?,
            Load::IOC => Set::LoadIOC.run(registers, memory).await?,
            Load::ToIONext => Set::IONext.run(registers, memory).await?,
            Load::IONext => Set::LoadIONext.run(registers, memory).await?,
        };
        Ok(cycles)
    }
}

#[cfg(test)]
mod test_instruction_load_reg_reg {
    use super::Load;
    use crate::executor;
    use crate::registers::{Bits16, Bits8, Bus};
    use crate::Registers;
    use memory::Memory;

    #[test]
    fn test_load_l_from_h() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load::HL;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory)));
        assert_eq!(
            register.borrow().get(Bits8::H),
            register.borrow().get(Bits8::L)
        );
    }

    #[test]
    fn test_load_b_from_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load::BHL;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::B),
            memory
                .borrow()
                .get_u8(register.borrow().get(Bits16::HL))
                .unwrap()
        );
    }

    #[test]
    fn test_load_reg_b_from_next_byte() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = Load::B8b;
        let byte = memory.borrow().get_u8(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        let future = ldr8b.exec(register.clone(), memory);
        executor::execute(Box::pin(future));
        assert_eq!(byte, register.borrow().get(Bits8::B));
    }

    #[test]
    fn test_load_hl_from_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load::HLB;
        register.borrow_mut().set(Bits16::HL, 0xc042);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::B),
            memory
                .borrow()
                .get_u8(register.borrow().get(Bits16::HL))
                .unwrap()
        );
    }

    #[test]
    fn test_load_hl_8b() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load::HL8b;
        let byte = memory.borrow().get_u8(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            byte,
            memory
                .borrow()
                .get_u8(register.borrow().get(Bits16::HL))
                .unwrap()
        );
    }
}
