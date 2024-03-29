use enum_iterator::IntoEnumIterator;

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

#[derive(Debug, IntoEnumIterator, PartialEq, Clone, Copy)]
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
