use super::super::area::Bits8;
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};

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
    ABC = 0x0A,
    ADE = 0x1A,
    AHL = 0x7E,
    ANN = 0xFA,
    ASHARP = 0x3E,
}

impl<'a> LoadRegANum8bit {
    pub fn proceed(self, registers: &'a mut Registers) -> Result<u32, Error> {
        match self {
            LoadRegANum8bit::AA => registers.
        }
    }
}
