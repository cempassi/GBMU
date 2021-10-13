pub(crate) mod arithmetic;
pub(crate) mod logical;

pub use arithmetic::{carry, Add, Sub};
pub use logical::{and, cmp, or, rotate_right, xor};

#[derive(Debug)]
pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
