use crate::area::{Bits16, Bits8};
use crate::pc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;

/// 1. LD nn,n
/// Description:
///  Put value nn into n.
/// Use with:
///  nn = A,B,C,D,E,H,L
///  n = 8 bit immediate value
/// Opcodes:
/// Instruction Parameters Opcode Cycles
/// LD          B,n        0x06   8
/// LD          C,n        0x0e   8
/// LD          D,n        0x16   8
/// LD          E,n        0x1e   8
/// LD          H,n        0x26   8
/// LD          L,n        0x2e   8
/// LD          L,n        0x2e   8
/// LD          (HL),n     0x36   12
/// LD          A,n        0x3e   8
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone)]
#[repr(u8)]
pub enum LoadR8b {
    B = 0x06,
    C = 0x0e,
    D = 0x16,
    E = 0x1e,
    H = 0x26,
    L = 0x2e,
    HL = 0x36,
    A = 0x3e,
}

impl LoadR8b {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let byte = registers.borrow_mut().pc.next(memory.clone()).unwrap();
        match self {
            LoadR8b::A => registers.borrow_mut().set(Bits8::A, byte),
            LoadR8b::B => registers.borrow_mut().set(Bits8::B, byte),
            LoadR8b::C => registers.borrow_mut().set(Bits8::C, byte),
            LoadR8b::D => registers.borrow_mut().set(Bits8::D, byte),
            LoadR8b::E => registers.borrow_mut().set(Bits8::E, byte),
            LoadR8b::H => registers.borrow_mut().set(Bits8::H, byte),
            LoadR8b::L => registers.borrow_mut().set(Bits8::L, byte),
            LoadR8b::HL => memory.borrow_mut().set(registers.borrow().get(Bits16::HL), byte).unwrap(),
        };
    }
}

#[cfg(test)]
mod test_instruction_load_8bit_into_reg {
    use memory::Memory;
    use crate::{RegisterBus, Registers};
    use crate::area::Bits8;
    use super::LoadR8b;

    #[test]
    fn test_reg_b() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = LoadR8b::B;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        ldr8b.exec(register.clone(), memory.clone());
        assert_eq!(byte, register.borrow().get(Bits8::B));
    }

    #[test]
    fn test_reg_c() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = LoadR8b::C;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        ldr8b.exec(register.clone(), memory.clone());
        assert_eq!(byte, register.borrow().get(Bits8::C));
    }

    #[test]
    fn test_reg_d() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = LoadR8b::D;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        ldr8b.exec(register.clone(), memory.clone());
        assert_eq!(byte, register.borrow().get(Bits8::D));
    }

    #[test]
    fn test_reg_e() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = LoadR8b::E;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        ldr8b.exec(register.clone(), memory.clone());
        assert_eq!(byte, register.borrow().get(Bits8::E));
    }

    #[test]
    fn test_reg_h() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = LoadR8b::H;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        ldr8b.exec(register.clone(), memory.clone());
        assert_eq!(byte, register.borrow().get(Bits8::H));
    }

    #[test]
    fn test_reg_l() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = LoadR8b::L;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        ldr8b.exec(register.clone(), memory.clone());
        assert_eq!(byte, register.borrow().get(Bits8::L));
        }

    #[test]
    fn test_reg_a() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldr8b = LoadR8b::A;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        ldr8b.exec(register.clone(), memory.clone());
        assert_eq!(byte, register.borrow().get(Bits8::A));
    }
}