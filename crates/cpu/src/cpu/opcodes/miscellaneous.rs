use super::super::area::{Bits16, Bits8, Flag};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};
use crate::cpu::flags::Flags;

///1. SWAP n
/// Description:
///  Swap upper & lower nibles of n.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Reset.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  SWAP A CB37 8
///  SWAP B CB30 8
///  SWAP C CB31 8
///  SWAP D CB32 8
///  SWAP E CB33 8
///  SWAP H CB34 8
///  SWAP L CB35 8
///  SWAP (HL) CB 36 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum SwapN {
    A = 0xcb37,
    B = 0xcb30,
    C = 0xcb31,
    D = 0xcb32,
    E = 0xcb33,
    H = 0xcb34,
    L = 0xcb35,
    HL = 0xcb36,
}

impl<'a> SwapN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let bits = match self {
                SwapN::A => Bits8::A,
                SwapN::B => Bits8::B,
                SwapN::C => Bits8::C,
                SwapN::D => Bits8::D,
                SwapN::E => Bits8::E,
                SwapN::H => Bits8::H,
                SwapN::L => Bits8::L,
                SwapN::HL => Bits16::HL,
            };
            let mut data = registers.get(bits);
            let upper_nibble = (data & 0xf0) >> 4;
            let lower_nibble = (data & 0x0f) << 4;
            dbg!("B4 {:?} UP {:?} LOW {:?}", data, upper_nibble, lower_nibble);
            data = lower_nibble | upper_nibble;
            dbg!("AFTER {:?}", data);
            registers.set(bits, data);
            Ok()
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

///2. DAA
/// Description:
///  Decimal adjust register A.
///  This instruction adjusts register A so that the
///  correct representation of Binary Coded Decimal (BCD)
///  is obtained.
/// Flags affected:
///  Z - Set if register A is zero.
///  N - Not affected.
///  H - Reset.
///  C - Set or reset according to operation.
///
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  DAA -/- 27 4
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum DecAdjustRegA {
//     DAA = 0x2f,
// }
//
// impl<'a> DecAdjustRegA {
//     pub fn proceed(
//         self,
//         registers: &'a mut Registers,
//         memory: &'a mut Memory,
//     ) -> Result<u32, Error> {
//         if let Ok(byte) = registers.pc.next(memory) {
//             // let mut data = registers.get(bits);
//             // let upper_nibble = (data & 0xf0) >> 4;
//             // let lower_nibble = (data & 0x0f) << 4;
//             // dbg!("B4 {:?} UP {:?} LOW {:?}", data, upper_nibble, lower_nibble);
//             // data = lower_nibble | upper_nibble;
//             // dbg!("AFTER {:?}", data);
//             // registers.set(bits, data);
//             // Ok()
//         } else {
//             Err(Error::InvalidPC(registers.pc))
//         }
//     }
// }

///3. CPL
/// Description:
///  Complement A register. (Flip all bits.)
/// Flags affected:
///  Z - Not affected.
///  N - Set.
///  H - Set.
///  C - Not affected.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  CPL -/- 2F 4
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum CplN {
//     CPL = 0x2f,
// }
//
// impl<'a> CplN {
//     pub fn proceed(
//         self,
//         registers: &'a mut Registers,
//         memory: &'a mut Memory,
//     ) -> Result<u32, Error> {
//         if let Ok(byte) = registers.pc.next(memory) {
//             memory.set(registers.pc)
//         } else {
//             Err(Error::InvalidPC(registers.pc))
//         }
//     }
// }

/// 4. CCF
/// Description:
///  Complement carry flag.
///  If C flag is set, then reset it.
///  If C flag is reset, then set it.
/// Flags affected:
///  Z - Not affected.
///  N - Reset.
///  H - Reset.
///  C - Complemented.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  CCF -/- 3F 4
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum CcfN {
//     CCF = 0x3f,
// }
//
// impl<'a> CcfN {
//     pub fn proceed(
//         self,
//         registers: &'a mut Registers,
//         memory: &'a mut Memory,
//     ) -> Result<u32, Error> {
//         if let Ok(byte) = registers.pc.next(memory) {
//             memory.set(registers.pc)
//         } else {
//             Err(Error::InvalidPC(registers.pc))
//         }
//     }
// }

/// 5. SCF
/// Description:
///  Set Carry flag.
/// Flags affected:
///  Z - Not affected.
///  N - Reset.
///  H - Reset.
///  C - Set.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  SCF -/- 37 4
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum ScfN {
//     SCF = 0x37,
// }
//
// impl<'a> ScfN {
//     pub fn proceed(
//         self,
//         registers: &'a mut Registers,
//         memory: &'a mut Memory,
//     ) -> Result<u32, Error> {
//         if let Ok(byte) = registers.pc.next(memory) {
//             todo!()
//                 register.set(Bits8::F, 0x40)
//         } else {
//             Err(Error::InvalidPC(registers.pc))
//         }
//     }
// }

/// 6. NOP
/// Description:
///  No operation.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  NOP -/- 00 4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Nop {
    NOP = 0x00,
}

impl<'a> CplN {
    pub fn proceed(
        self,
    ) -> Result<u32, Error> {
        Ok()
    }
}

/// 7. HALT
/// Description:
///  Power down CPU until an interrupt occurs. Use this
///  when ever possible to reduce energy consumption.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  HALT -/- 76 4
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum Halt {
//     HALT = 0x76,
// }
//
// impl<'a> Halt {
//     pub fn proceed(
//         self,
//         registers: &'a mut Registers,
//     ) -> Result<u32, Error> {
//         todo!()
//     }
// }

/// 8. STOP
/// Description:
///  Halt CPU & LCD display until button pressed.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  STOP -/-  1000 4
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum Stop {
//     STOP = 0x1000,
// }
//
// impl<'a> Stop {
//     pub fn proceed(
//         self,
//         registers: &'a mut Registers,
//     ) -> Result<u32, Error> {
//         registers.halt = true
//     }
// }

/// 9. DI
/// Description:
///  This instruction disables interrupts but not
///  immediately. Interrupts are disabled after
///  instruction after DI is executed.
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  DI -/- F3 4
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum DisableInterrupts {
//     DI = 0xfb,
// }
//
// impl<'a> DisableInterrupts {
//     pub fn proceed(
//         self,
//         registers: &'a mut Registers,
//     ) -> Result<u32, Error> {
//         registers.en_interrupts = false
//     }
// }

/// 10. EI
/// Description:
///  Enable interrupts. This intruction enables interrupts
///  but not immediately. Interrupts are enabled after
///  instruction after EI is executed.
/// Flags affected:
///  None.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  EI -/- FB 4
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(u8)]
// pub enum EnableInterrupts {
//     EI = 0xfb,
// }
//
// impl<'a> CplN {
//     pub fn proceed(
//         self,
//         registers: &'a mut Registers,
//     ) -> Result<u32, Error> {
//         registers.en_interrupts = true
//     }
// }