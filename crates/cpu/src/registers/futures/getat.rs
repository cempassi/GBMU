use super::Reader;
use crate::registers::Bits16;
use crate::{Bus, Registers};
use memory::{Async, Memory};
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Getter<T> = Pin<Box<dyn Future<Output = Result<T, Error>>>>;

pub(crate) trait GetAt<T> {
    fn get_at(self, memory: Memory, area: Bits16) -> Getter<T>;
}

impl GetAt<u8> for Registers {
    fn get_at(self, memory: Memory, area: Bits16) -> Getter<u8> {
        let address = self.borrow().get(area);
        let inner = Box::pin(memory.get::<u8>(address));
        Box::pin(Reader::new(inner))
    }
}

impl GetAt<u16> for Registers {
    fn get_at(self, memory: Memory, area: Bits16) -> Getter<u16> {
        let address = self.borrow().get(area);
        let inner = Box::pin(memory.get::<u16>(address));
        Box::pin(Reader::new(inner))
    }
}
