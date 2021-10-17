pub(crate) mod area;
pub(crate) mod shift;
pub(crate) mod arithmetic;
pub(crate) mod logical;
pub(crate) mod flags;
pub(crate) mod futures;

pub(crate) use flags::Flags;
pub use area::{Bits8, Bits16, Flag};
pub(crate) use shift::Shift;
pub(crate) use arithmetic::Arithmetic;
pub(crate) use logical::Logical;

#[derive(Debug, Default)]
pub struct Registers {
    pub(crate) a: u8,
    pub(crate) f: Flags,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) h: u8,
    pub(crate) l: u8,
    pub(crate) sp: u16,
    pub pc: u16,
}
