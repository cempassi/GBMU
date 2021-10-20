use crate::registers::{Bits16, Bus};
use crate::Registers;
use memory::{Async, Memory};
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Setter = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

pub(crate) trait SetAt<T> {
    fn set_at(self, memory: Memory, area: Bits16, data: T) -> Setter;
}

impl SetAt<u8> for Registers {
    fn set_at(self, memory: Memory, area: Bits16, data: u8) -> Setter {
        let address = self.borrow().get(area);
        Box::pin(memory.set::<u8>(address, data))
    }
}

impl SetAt<u16> for Registers {
    fn set_at(self, memory: Memory, area: Bits16, data: u16) -> Setter {
        let address = self.borrow().get(area);
        Box::pin(memory.set::<u16>(address, data))
    }
}
