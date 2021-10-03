use super::futures::{Getter, Setter};
use crate::Memory;

pub trait Async {
    fn get(self, address: u16) -> Getter;
    fn set(self, address: u16, data: u8) -> Setter;
}

impl Async for Memory {
    fn get(self, address: u16) -> Getter {
        Getter::new(self, address)
    }

    fn set(self, address: u16, data: u8) -> Setter {
        Setter::new(self, address, data)
    }
}
