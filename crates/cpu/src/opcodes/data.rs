pub(crate) mod arithmetic;
pub(crate) mod logical;

pub use arithmetic::Add;
pub use logical::{and, or};

pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
