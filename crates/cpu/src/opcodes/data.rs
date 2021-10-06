pub(crate) mod arithmetic;
mod consts;
pub(crate) mod logical;

pub use arithmetic::Add;

pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
