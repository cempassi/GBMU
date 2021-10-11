pub(crate) mod arithmetic;
pub(crate) mod logical;

pub use arithmetic::Add;
pub use logical::and;

pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
