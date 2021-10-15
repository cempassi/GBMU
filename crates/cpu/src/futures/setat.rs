use crate::area::Bits16;
use crate::{Reader, RegisterBus, Registers};
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
        let inner = Box::pin(memory.set::<u8>(address, data));
        Box::pin(Reader::new(inner))
    }
}

impl SetAt<u16> for Registers {
    fn set_at(self, memory: Memory, area: Bits16, data: u16) -> Setter {
        let address = self.borrow().get(area);
        let inner = Box::pin(memory.set::<u16>(address, data));
        Box::pin(Reader::new(inner))
    }
}
