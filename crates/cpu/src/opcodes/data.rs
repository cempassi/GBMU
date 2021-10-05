pub(crate) mod arithmetic;
pub mod logical;

pub use arithmetic::Add;
pub use logical::or;

pub enum Data<T> {
    Carry(T),
    NoCarry(T),
}
