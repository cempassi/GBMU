use super::futures::{Getter, Setter};
use crate::Memory;

pub trait Async {
    fn get<T>(self, address: u16) -> Getter<T>;
    fn set<T>(self, address: u16, data: T) -> Setter<T>;
}

impl Async for Memory {
    fn get<T>(self, address: u16) -> Getter<T> {
        Getter::new(self, address)
    }

    fn set<T>(self, address: u16, data: T) -> Setter<T> {
        Setter::new(self, address, data)
    }
}
