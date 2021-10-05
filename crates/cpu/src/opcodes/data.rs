pub(crate) mod arithmetic;
pub(crate) mod logical;

pub use arithmetic::Add;

pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
