use super::futures::{Getter, Setter};
use crate::Memory;

pub trait Async<T> {
    fn get(self, address: u16) -> Getter<T>;
    fn set(self, address: u16, data: T) -> Setter<T>;
}

impl Async<u8> for Memory {
    fn get(self, address: u16) -> Getter<u8> {
        Getter::new(self, address, 0)
    }

    fn set(self, address: u16, data: u8) -> Setter<u8> {
        Setter::new(self, address, data)
    }
}

impl Async<u16> for Memory {
    fn get(self, address: u16) -> Getter<u16> {
        Getter::new(self, address, 0)
    }

    fn set(self, address: u16, data: u16) -> Setter<u16> {
        Setter::new(self, address, data)
    }
}
