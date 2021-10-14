use enum_iterator::IntoEnumIterator;
use std::convert::From;

#[derive(Debug, IntoEnumIterator, PartialEq, Clone, Copy)]
pub enum Bits8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Clone, Copy)]
pub enum Bits16 {
    AF,
    SP,
    PC,
    BC,
    DE,
    HL,
}

impl From<Bits8> for Bits16 {
    fn from(bit8: Bits8) -> Self {
        match bit8 {
            Bits8::A => Bits16::AF,
            Bits8::F => Bits16::AF,
            Bits8::B => Bits16::BC,
            Bits8::C => Bits16::BC,
            Bits8::D => Bits16::DE,
            Bits8::E => Bits16::DE,
            Bits8::H => Bits16::HL,
            Bits8::L => Bits16::HL,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    /// Zero flag
    /// This flag is set when :
    /// - the result of a math op is zero
    /// - `Cmp` OP match 2 values
    Z,

    /// Substract Flag
    /// This flag is set when the last math instruction was a substraction
    N,

    /// Half Carry Flag
    /// This flag is set when a carry occurred in the lower nibble of the last math OP
    H,

    /// Carry Flag
    /// This flag is set when :
    /// - a carry occurred in the last math OP
    /// - Reg A is the smaller value when doing a `Cmp` OP
    C,
}
