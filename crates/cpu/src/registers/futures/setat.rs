use super::{NextPc, Reader};
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

pub(crate) trait SetData {
    fn set_data(self, memory: Memory, area: Bits16) -> Setter;
}

async fn set_data(registers: Registers, memory: Memory, area: Bits16) -> Result<(), Error> {
    let data: u16 = registers.borrow().get(area);
    let dst: u16 = registers.next_pc(memory.clone()).await?;
    memory.set(dst, data).await?;
    Ok(())
}

impl SetData for Registers {
    fn set_data(self, memory: Memory, area: Bits16) -> Setter {
        let inner = Box::pin(set_data(self, memory, area));
        Box::pin(Reader::new(inner))
    }
}
