pub(crate) mod arithmetic;
pub(crate) mod logical;

pub use arithmetic::{Add, Sub};
pub use logical::{and, or, xor};

pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
