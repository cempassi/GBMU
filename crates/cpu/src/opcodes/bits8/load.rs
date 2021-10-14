use crate::area::{Bits16, Bits8};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::futures::GetAt;
use crate::nextpc::NextPc;
use memory::Memory;
use num_enum::TryFromPrimitive;

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
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadRegister {
    B = 0x06,
    C = 0x0e,
    D = 0x16,
    E = 0x1e,
    H = 0x26,
    L = 0x2e,
    A = 0x3e,
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
}

enum Load {
    Register,
    Pointer,
    Next,
}

impl Load {
    pub async fn load(self, registers: Registers, memory: Memory, src: Option<Bits8>, dst: Bits8) {
        let data = match self {
            Load::Register => registers.borrow().get(src.unwrap()),
            Load::Pointer => registers.clone().get_at(memory, Bits16::HL).await.unwrap(),
            Load::Next => registers.clone().next_pc(memory).await.unwrap(),
        };
        registers.borrow_mut().set(dst, data);
    }
}

impl LoadRegister {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            LoadRegister::AA => Load::Register.load(registers, memory, Some(Bits8::A), Bits8::A),
            LoadRegister::AB => Load::Register.load(registers, memory, Some(Bits8::A), Bits8::B),
            LoadRegister::AC => Load::Register.load(registers, memory, Some(Bits8::A), Bits8::C),
            LoadRegister::AD => Load::Register.load(registers, memory, Some(Bits8::A), Bits8::D),
            LoadRegister::AE => Load::Register.load(registers, memory, Some(Bits8::A), Bits8::E),
            LoadRegister::AH => Load::Register.load(registers, memory, Some(Bits8::A), Bits8::H),
            LoadRegister::AL => Load::Register.load(registers, memory, Some(Bits8::A), Bits8::L),
            LoadRegister::BB => Load::Register.load(registers, memory, Some(Bits8::B), Bits8::B),
            LoadRegister::BC => Load::Register.load(registers, memory, Some(Bits8::B), Bits8::C),
            LoadRegister::BD => Load::Register.load(registers, memory, Some(Bits8::B), Bits8::D),
            LoadRegister::BE => Load::Register.load(registers, memory, Some(Bits8::B), Bits8::E),
            LoadRegister::BH => Load::Register.load(registers, memory, Some(Bits8::B), Bits8::H),
            LoadRegister::BL => Load::Register.load(registers, memory, Some(Bits8::B), Bits8::L),
            LoadRegister::CB => Load::Register.load(registers, memory, Some(Bits8::C), Bits8::B),
            LoadRegister::CC => Load::Register.load(registers, memory, Some(Bits8::C), Bits8::C),
            LoadRegister::CD => Load::Register.load(registers, memory, Some(Bits8::C), Bits8::D),
            LoadRegister::CE => Load::Register.load(registers, memory, Some(Bits8::C), Bits8::E),
            LoadRegister::CH => Load::Register.load(registers, memory, Some(Bits8::C), Bits8::H),
            LoadRegister::CL => Load::Register.load(registers, memory, Some(Bits8::C), Bits8::L),
            LoadRegister::DB => Load::Register.load(registers, memory, Some(Bits8::D), Bits8::B),
            LoadRegister::DC => Load::Register.load(registers, memory, Some(Bits8::D), Bits8::C),
            LoadRegister::DD => Load::Register.load(registers, memory, Some(Bits8::D), Bits8::D),
            LoadRegister::DE => Load::Register.load(registers, memory, Some(Bits8::D), Bits8::E),
            LoadRegister::DH => Load::Register.load(registers, memory, Some(Bits8::D), Bits8::H),
            LoadRegister::DL => Load::Register.load(registers, memory, Some(Bits8::D), Bits8::L),
            LoadRegister::EB => Load::Register.load(registers, memory, Some(Bits8::E), Bits8::B),
            LoadRegister::EC => Load::Register.load(registers, memory, Some(Bits8::E), Bits8::C),
            LoadRegister::ED => Load::Register.load(registers, memory, Some(Bits8::E), Bits8::D),
            LoadRegister::EE => Load::Register.load(registers, memory, Some(Bits8::E), Bits8::E),
            LoadRegister::EH => Load::Register.load(registers, memory, Some(Bits8::E), Bits8::H),
            LoadRegister::EL => Load::Register.load(registers, memory, Some(Bits8::E), Bits8::L),
            LoadRegister::HB => Load::Register.load(registers, memory, Some(Bits8::H), Bits8::B),
            LoadRegister::HC => Load::Register.load(registers, memory, Some(Bits8::H), Bits8::C),
            LoadRegister::HD => Load::Register.load(registers, memory, Some(Bits8::H), Bits8::D),
            LoadRegister::HE => Load::Register.load(registers, memory, Some(Bits8::H), Bits8::E),
            LoadRegister::HH => Load::Register.load(registers, memory, Some(Bits8::H), Bits8::H),
            LoadRegister::HL => Load::Register.load(registers, memory, Some(Bits8::H), Bits8::L),
            LoadRegister::LB => Load::Register.load(registers, memory, Some(Bits8::L), Bits8::B),
            LoadRegister::LC => Load::Register.load(registers, memory, Some(Bits8::L), Bits8::C),
            LoadRegister::LD => Load::Register.load(registers, memory, Some(Bits8::L), Bits8::D),
            LoadRegister::LE => Load::Register.load(registers, memory, Some(Bits8::L), Bits8::E),
            LoadRegister::LH => Load::Register.load(registers, memory, Some(Bits8::L), Bits8::H),
            LoadRegister::LL => Load::Register.load(registers, memory, Some(Bits8::L), Bits8::L),
            LoadRegister::BHL => Load::Pointer.load(registers, memory, None, Bits8::B),
            LoadRegister::CHL => Load::Pointer.load(registers, memory, None, Bits8::C),
            LoadRegister::DHL => Load::Pointer.load(registers, memory, None, Bits8::D),
            LoadRegister::EHL => Load::Pointer.load(registers, memory, None, Bits8::E),
            LoadRegister::HHL => Load::Pointer.load(registers, memory, None, Bits8::H),
            LoadRegister::LHL => Load::Pointer.load(registers, memory, None, Bits8::L),
            LoadRegister::AHL => Load::Pointer.load(registers, memory, None, Bits8::A),
            LoadRegister::B => Load::Next.load(registers, memory, None, Bits8::B),
            LoadRegister::C => Load::Next.load(registers, memory, None, Bits8::C),
            LoadRegister::D => Load::Next.load(registers, memory, None, Bits8::D),
            LoadRegister::E => Load::Next.load(registers, memory, None, Bits8::E),
            LoadRegister::H => Load::Next.load(registers, memory, None, Bits8::H),
            LoadRegister::L => Load::Next.load(registers, memory, None, Bits8::L),
            LoadRegister::A => Load::Next.load(registers, memory, None, Bits8::A),
        }
        .await;
    }
}

#[cfg(test)]
mod test_instruction_load_reg_reg {
    use super::LoadRegister;
    use crate::area::{Bits16, Bits8};
    use crate::executor;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_l_from_h() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegister::HL;
        executor::execute(Box::pin(instruction.exec(register.clone(), memory.clone())));
        assert_eq!(
            register.borrow().get(Bits8::H),
            register.borrow().get(Bits8::L)
        );
    }

    #[test]
    fn test_load_b_from_hl() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = LoadRegister::BHL;
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
    fn test_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = LoadRegister::B;
        let byte = memory.borrow().get_u8(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        let future = ldr8b.exec(register.clone(), memory.clone());
        executor::execute(Box::pin(future));
        assert_eq!(byte, register.borrow().get(Bits8::B));
    }
}
