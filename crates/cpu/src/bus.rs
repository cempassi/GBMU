use std::fmt::Debug;

pub trait RegisterBus<T, O>: Debug {
    fn get(&self, _: T) -> O;

    fn set(&mut self, _: T, data: O);
}
