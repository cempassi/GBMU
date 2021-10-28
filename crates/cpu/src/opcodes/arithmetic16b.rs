use super::decode::{Decode, Decoder};
use crate::registers::{
    futures::{AsyncGet, Get},
    Arithmetic, Bits16, IncDec,
};
use crate::{Access, Cpu};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use shared::Error;
use std::fmt;

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

#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
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
    fn decode(self, cpu: Cpu) -> Decode {
        Box::pin(self.exec(cpu))
    }
}
impl Arithmetic16b {
    pub async fn exec(self, cpu: Cpu) -> Result<u8, Error> {
        match self {
            Arithmetic16b::IncBC => cpu.registers().borrow_mut().increase(Bits16::BC, 1),
            Arithmetic16b::IncDE => cpu.registers().borrow_mut().increase(Bits16::DE, 1),
            Arithmetic16b::IncHL => cpu.registers().borrow_mut().increase(Bits16::HL, 1),
            Arithmetic16b::IncSP => cpu.registers().borrow_mut().increase(Bits16::SP, 1),
            Arithmetic16b::DecBC => cpu.registers().borrow_mut().decrease(Bits16::BC, 1),
            Arithmetic16b::DecDE => cpu.registers().borrow_mut().decrease(Bits16::DE, 1),
            Arithmetic16b::DecHL => cpu.registers().borrow_mut().decrease(Bits16::HL, 1),
            Arithmetic16b::DecSP => cpu.registers().borrow_mut().decrease(Bits16::SP, 1),
            Arithmetic16b::AddBC => cpu.registers().borrow_mut().add(Bits16::BC, false),
            Arithmetic16b::AddDE => cpu.registers().borrow_mut().add(Bits16::DE, false),
            Arithmetic16b::AddHL => cpu.registers().borrow_mut().add(Bits16::HL, false),
            Arithmetic16b::AddSP => cpu.registers().borrow_mut().add(Bits16::SP, false),
        };
        let (_, cycles): (u8, u8) = Get::Nop.get(cpu).await?;
        Ok(cycles)
    }
}

impl fmt::Display for Arithmetic16b {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arithmetic16b::IncBC => write!(f, "Increase BC"),
            Arithmetic16b::IncDE => write!(f, "Increase DE"),
            Arithmetic16b::IncHL => write!(f, "Increase HL"),
            Arithmetic16b::IncSP => write!(f, "Increase SP"),
            Arithmetic16b::DecBC => write!(f, "Decrease BC"),
            Arithmetic16b::DecDE => write!(f, "Decrease DE"),
            Arithmetic16b::DecHL => write!(f, "Decrease HL"),
            Arithmetic16b::DecSP => write!(f, "Decrease SP"),
            Arithmetic16b::AddBC => write!(f, "Add HL BC"),
            Arithmetic16b::AddDE => write!(f, "Add HL DE"),
            Arithmetic16b::AddHL => write!(f, "Add HL HL"),
            Arithmetic16b::AddSP => write!(f, "Add HL SP"),
        }
    }
}
