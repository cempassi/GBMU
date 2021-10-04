pub(crate) mod arithmetic;

pub use arithmetic::Add;

pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
