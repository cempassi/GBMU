pub(crate) mod area;
pub(crate) mod arithmetic;
pub(crate) mod bitwise;
pub mod bus;
pub(crate) mod flags;
pub(crate) mod futures;
pub(crate) mod jump;
pub(crate) mod load;
pub(crate) mod logical;
pub(crate) mod rotation;
pub(crate) mod shift;

pub use area::{Bits16, Bits8, Flag};
pub(crate) use arithmetic::Arithmetic;
pub(crate) use bitwise::Bitwise;
pub use bus::Bus;
pub(crate) use flags::Flags;
pub(crate) use jump::{Absolute, Relative};
pub(crate) use load::Load;
pub(crate) use logical::Logical;
pub(crate) use rotation::Rotation;
pub(crate) use shift::Shift;

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
