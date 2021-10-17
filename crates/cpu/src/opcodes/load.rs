use crate::area::{Bits16, Bits8};
use crate::bus::RegisterBus;
use crate::cpu::Registers;
use crate::futures::{GetAt, SetAt};
use crate::nextpc::NextPc;
use crate::Arithmetic;
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
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Load {
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
}

#[derive(Clone, Copy)]
enum Src {
    Register(Bits8),
    Pointer(Bits16),
    Next,
    Increase,
    Decrease,
}

enum Dst {
    Register(Bits8),
    Pointer(Bits16),
    Increase(Bits16),
    Decrease(Bits16),
}

impl Dst {
    pub async fn load(self, source: Src, registers: Registers, memory: Memory) {
        let data = match source {
            Src::Register(src) => registers.borrow().get(src),
            Src::Pointer(src) => registers.clone().get_at(memory.clone(), src).await.unwrap(),
            Src::Next => registers.clone().next_pc(memory.clone()).await.unwrap(),
            Src::Increase => {
                let data = registers
                    .clone()
                    .get_at(memory.clone(), Bits16::HL)
                    .await
                    .unwrap();
                registers.borrow_mut().increase(Bits16::HL, 1);
                data
            }
            Src::Decrease => {
                let data = registers
                    .clone()
                    .get_at(memory.clone(), Bits16::HL)
                    .await
                    .unwrap();
                registers.borrow_mut().decrease(Bits16::HL, 1);
                data
            }
        };
        match self {
            Dst::Register(dst) => registers.borrow_mut().set(dst, data),
            Dst::Pointer(dst) => registers.set_at(memory, dst, data).await.unwrap(),
            Dst::Increase(dst) => {
                registers.borrow_mut().increase(Bits16::HL, 1);
                registers.set_at(memory, dst, data).await.unwrap();
            }
            Dst::Decrease(dst) => {
                registers.borrow_mut().decrease(Bits16::HL, 1);
                registers.set_at(memory, dst, data).await.unwrap();
            }
        };
    }
}

impl Load {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        match self {
            Load::B => Dst::Register(Bits8::B).load(Src::Next, registers, memory),
            Load::C => Dst::Register(Bits8::C).load(Src::Next, registers, memory),
            Load::D => Dst::Register(Bits8::D).load(Src::Next, registers, memory),
            Load::E => Dst::Register(Bits8::E).load(Src::Next, registers, memory),
            Load::H => Dst::Register(Bits8::H).load(Src::Next, registers, memory),
            Load::L => Dst::Register(Bits8::L).load(Src::Next, registers, memory),
            Load::A => Dst::Register(Bits8::A).load(Src::Next, registers, memory),
            Load::HL8b => Dst::Pointer(Bits16::HL).load(Src::Next, registers, memory),
            Load::BHL => Dst::Register(Bits8::B).load(Src::Pointer(Bits16::HL), registers, memory),
            Load::CHL => Dst::Register(Bits8::C).load(Src::Pointer(Bits16::HL), registers, memory),
            Load::DHL => Dst::Register(Bits8::D).load(Src::Pointer(Bits16::HL), registers, memory),
            Load::EHL => Dst::Register(Bits8::E).load(Src::Pointer(Bits16::HL), registers, memory),
            Load::HHL => Dst::Register(Bits8::H).load(Src::Pointer(Bits16::HL), registers, memory),
            Load::LHL => Dst::Register(Bits8::L).load(Src::Pointer(Bits16::HL), registers, memory),
            Load::AHL => Dst::Register(Bits8::A).load(Src::Pointer(Bits16::HL), registers, memory),
            Load::AA => Dst::Register(Bits8::A).load(Src::Register(Bits8::A), registers, memory),
            Load::AB => Dst::Register(Bits8::B).load(Src::Register(Bits8::A), registers, memory),
            Load::AC => Dst::Register(Bits8::C).load(Src::Register(Bits8::A), registers, memory),
            Load::AD => Dst::Register(Bits8::D).load(Src::Register(Bits8::A), registers, memory),
            Load::AE => Dst::Register(Bits8::E).load(Src::Register(Bits8::A), registers, memory),
            Load::AH => Dst::Register(Bits8::H).load(Src::Register(Bits8::A), registers, memory),
            Load::AL => Dst::Register(Bits8::L).load(Src::Register(Bits8::A), registers, memory),
            Load::BB => Dst::Register(Bits8::B).load(Src::Register(Bits8::B), registers, memory),
            Load::BC => Dst::Register(Bits8::C).load(Src::Register(Bits8::B), registers, memory),
            Load::BD => Dst::Register(Bits8::D).load(Src::Register(Bits8::B), registers, memory),
            Load::BE => Dst::Register(Bits8::E).load(Src::Register(Bits8::B), registers, memory),
            Load::BH => Dst::Register(Bits8::H).load(Src::Register(Bits8::B), registers, memory),
            Load::BL => Dst::Register(Bits8::L).load(Src::Register(Bits8::B), registers, memory),
            Load::CB => Dst::Register(Bits8::B).load(Src::Register(Bits8::C), registers, memory),
            Load::CC => Dst::Register(Bits8::C).load(Src::Register(Bits8::C), registers, memory),
            Load::CD => Dst::Register(Bits8::D).load(Src::Register(Bits8::C), registers, memory),
            Load::CE => Dst::Register(Bits8::E).load(Src::Register(Bits8::C), registers, memory),
            Load::CH => Dst::Register(Bits8::H).load(Src::Register(Bits8::C), registers, memory),
            Load::CL => Dst::Register(Bits8::L).load(Src::Register(Bits8::C), registers, memory),
            Load::DB => Dst::Register(Bits8::B).load(Src::Register(Bits8::D), registers, memory),
            Load::DC => Dst::Register(Bits8::C).load(Src::Register(Bits8::D), registers, memory),
            Load::DD => Dst::Register(Bits8::D).load(Src::Register(Bits8::D), registers, memory),
            Load::DE => Dst::Register(Bits8::E).load(Src::Register(Bits8::D), registers, memory),
            Load::DH => Dst::Register(Bits8::H).load(Src::Register(Bits8::D), registers, memory),
            Load::DL => Dst::Register(Bits8::L).load(Src::Register(Bits8::D), registers, memory),
            Load::EB => Dst::Register(Bits8::B).load(Src::Register(Bits8::E), registers, memory),
            Load::EC => Dst::Register(Bits8::C).load(Src::Register(Bits8::E), registers, memory),
            Load::ED => Dst::Register(Bits8::D).load(Src::Register(Bits8::E), registers, memory),
            Load::EE => Dst::Register(Bits8::E).load(Src::Register(Bits8::E), registers, memory),
            Load::EH => Dst::Register(Bits8::H).load(Src::Register(Bits8::E), registers, memory),
            Load::EL => Dst::Register(Bits8::L).load(Src::Register(Bits8::E), registers, memory),
            Load::HB => Dst::Register(Bits8::B).load(Src::Register(Bits8::H), registers, memory),
            Load::HC => Dst::Register(Bits8::C).load(Src::Register(Bits8::H), registers, memory),
            Load::HD => Dst::Register(Bits8::D).load(Src::Register(Bits8::H), registers, memory),
            Load::HE => Dst::Register(Bits8::E).load(Src::Register(Bits8::H), registers, memory),
            Load::HH => Dst::Register(Bits8::H).load(Src::Register(Bits8::H), registers, memory),
            Load::HL => Dst::Register(Bits8::L).load(Src::Register(Bits8::H), registers, memory),
            Load::LB => Dst::Register(Bits8::B).load(Src::Register(Bits8::L), registers, memory),
            Load::LC => Dst::Register(Bits8::C).load(Src::Register(Bits8::L), registers, memory),
            Load::LD => Dst::Register(Bits8::D).load(Src::Register(Bits8::L), registers, memory),
            Load::LE => Dst::Register(Bits8::E).load(Src::Register(Bits8::L), registers, memory),
            Load::LH => Dst::Register(Bits8::H).load(Src::Register(Bits8::L), registers, memory),
            Load::LL => Dst::Register(Bits8::L).load(Src::Register(Bits8::L), registers, memory),
            Load::HLB => Dst::Pointer(Bits16::HL).load(Src::Register(Bits8::B), registers, memory),
            Load::HLC => Dst::Pointer(Bits16::HL).load(Src::Register(Bits8::C), registers, memory),
            Load::HLD => Dst::Pointer(Bits16::HL).load(Src::Register(Bits8::D), registers, memory),
            Load::HLE => Dst::Pointer(Bits16::HL).load(Src::Register(Bits8::E), registers, memory),
            Load::HLH => Dst::Pointer(Bits16::HL).load(Src::Register(Bits8::H), registers, memory),
            Load::HLL => Dst::Pointer(Bits16::HL).load(Src::Register(Bits8::L), registers, memory),
            Load::HLA => Dst::Pointer(Bits16::HL).load(Src::Register(Bits8::A), registers, memory),
            Load::BCA => Dst::Pointer(Bits16::BC).load(Src::Register(Bits8::A), registers, memory),
            Load::DEA => Dst::Pointer(Bits16::DE).load(Src::Register(Bits8::A), registers, memory),
            Load::ABC => Dst::Register(Bits8::A).load(Src::Pointer(Bits16::BC), registers, memory),
            Load::ADE => Dst::Register(Bits8::A).load(Src::Pointer(Bits16::DE), registers, memory),
            Load::HLPA => {
                Dst::Increase(Bits16::HL).load(Src::Register(Bits8::A), registers, memory)
            }
            Load::HLMA => {
                Dst::Decrease(Bits16::HL).load(Src::Register(Bits8::A), registers, memory)
            }
            Load::AHLP => Dst::Register(Bits8::A).load(Src::Increase, registers, memory),
            Load::AHLM => Dst::Register(Bits8::A).load(Src::Decrease, registers, memory),
        }
        .await;
    }
}

#[cfg(test)]
mod test_instruction_load_reg_reg {
    use super::Load;
    use crate::area::{Bits16, Bits8};
    use crate::executor;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_l_from_h() {
        let register = Registers::default();
        let memory = Memory::default();
        let instruction = Load::HL;
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
        let ldr8b = Load::B;
        let byte = memory.borrow().get_u8(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        let future = ldr8b.exec(register.clone(), memory.clone());
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
