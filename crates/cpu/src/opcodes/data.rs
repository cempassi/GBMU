pub(crate) mod arithmetic;
pub(crate) mod logical;

pub use arithmetic::{Add, Sub};
pub use logical::{and, cmp, or, xor};

#[derive(Debug)]
pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
