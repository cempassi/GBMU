pub(crate) mod arithmetic;
pub(crate) mod consts;

pub use arithmetic::Add;

pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
