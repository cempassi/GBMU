use crate::{Reader, Registers, RegisterBus};
use memory::{Memory, Async};
use super::area::Bits16;
use shared::Error;
use std::pin::Pin;
use std::future::Future;

type F =  Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

pub(crate) trait GetAt {
    fn get_at(self, memory: Memory, area: Bits16) -> F;
}

impl GetAt for Registers {
    fn get_at(self, memory: Memory, area: Bits16) -> F {
        let inner = Box::pin(get_at(self, memory, area));
        Box::pin(Reader::new(inner))
    }
}

async fn get_at(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let address = registers.borrow().get(area);
    memory.get::<u8>(address).await
}
