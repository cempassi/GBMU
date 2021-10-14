use super::area::Bits16;
use crate::{Reader, RegisterBus, Registers};
use memory::{Async, Memory};
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Getter = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;
type Setter = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

pub(crate) trait GetAt {
    fn get_at(self, memory: Memory, area: Bits16) -> Getter;
}

impl GetAt for Registers {
    fn get_at(self, memory: Memory, area: Bits16) -> Getter {
        let inner = Box::pin(get_at(self, memory, area));
        Box::pin(Reader::new(inner))
    }
}

async fn get_at(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let address = registers.borrow().get(area);
    memory.get::<u8>(address).await
}

pub(crate) trait SetAt {
    fn set_at(self, memory: Memory, area: Bits16, data: u8) -> Setter;
}

impl SetAt for Registers {
    fn set_at(self, memory: Memory, area: Bits16, data: u8) -> Setter {
        let inner = Box::pin(set_at(self, memory, area, data));
        Box::pin(Reader::new(inner))
    }
}

async fn set_at(registers: Registers, memory: Memory, area: Bits16, data: u8) -> Result<(), Error> {
    let address = registers.borrow().get(area);
    memory.set::<u8>(address, data).await
}
