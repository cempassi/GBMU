use super::decode::{Decode, Decoder};
use crate::cpu::Registers;
use crate::registers::{
    futures::{AsyncGet, Get},
    Arithmetic, Bits16, IncDec,
};
use memory::Memory;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;

/// ADD HL,r16
/// Add the value in r16 to HL.
///
/// Cycles: 8
/// Bytes: 1
/// Flags:
///
/// N - 0
/// H - Set if overflow from bit 11.
/// C - Set if overflow from bit 15.
///
///
/// DEC r16
/// Decrement value in register r16 by 1.
///
/// Cycles: 8
/// Bytes: 1
/// Flags: None affected.
///
///
/// INC r16
/// Increment value in register r16 by 1.
///
/// Cycles: 8
///
/// Bytes: 1
/// Flags: None affected.

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Arithmetic16b {
    IncBC = 0x03,
    IncDE = 0x13,
    IncHL = 0x23,
    IncSP = 0x33,
    DecBC = 0x0B,
    DecDE = 0x1B,
    DecHL = 0x2B,
    DecSP = 0x3B,
    AddBC = 0x09,
    AddDE = 0x19,
    AddHL = 0x29,
    AddSP = 0x39,
}

impl Decoder for Arithmetic16b {
    fn decode(self, registers: Registers, memory: Memory) -> Decode {
        Box::pin(self.exec(registers, memory))
    }
}

impl Arithmetic16b {
    pub async fn exec(self, registers: Registers, memory: Memory) -> Result<u8, Error> {
        match self {
            Arithmetic16b::IncBC => registers.borrow_mut().increase(Bits16::BC, 1),
            Arithmetic16b::IncDE => registers.borrow_mut().increase(Bits16::DE, 1),
            Arithmetic16b::IncHL => registers.borrow_mut().increase(Bits16::HL, 1),
            Arithmetic16b::IncSP => registers.borrow_mut().increase(Bits16::SP, 1),
            Arithmetic16b::DecBC => registers.borrow_mut().decrease(Bits16::BC, 1),
            Arithmetic16b::DecDE => registers.borrow_mut().decrease(Bits16::DE, 1),
            Arithmetic16b::DecHL => registers.borrow_mut().decrease(Bits16::HL, 1),
            Arithmetic16b::DecSP => registers.borrow_mut().decrease(Bits16::SP, 1),
            Arithmetic16b::AddBC => registers.borrow_mut().add(Bits16::BC, false),
            Arithmetic16b::AddDE => registers.borrow_mut().add(Bits16::DE, false),
            Arithmetic16b::AddHL => registers.borrow_mut().add(Bits16::HL, false),
            Arithmetic16b::AddSP => registers.borrow_mut().add(Bits16::SP, false),
        };
        let (_, cycles): (u8, u8) = Get::Nop.get(registers, memory).await?;
        Ok(cycles)
    }
}
