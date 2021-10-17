use super::{NextPc, Reader};
use crate::registers::{Bits16, Bits8};
use crate::{Bus, Registers};
use memory::{Async, Memory};
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Loader = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

pub(crate) trait LoadData<T> {
    fn load_data(self, memory: Memory, area: T) -> Loader;
}

async fn load_u8(registers: Registers, memory: Memory, area: Bits8) -> Result<(), Error> {
    let data = registers.clone().next_pc(memory.clone()).await?;
    registers.borrow_mut().set(area, data);
    Ok(())
}

async fn load_u16(registers: Registers, memory: Memory, area: Bits16) -> Result<(), Error> {
    let data = registers.clone().next_pc(memory.clone()).await?;
    registers.borrow_mut().set(area, data);
    Ok(())
}

impl LoadData<Bits8> for Registers {
    fn load_data(self, memory: Memory, area: Bits8) -> Loader {
        let inner = Box::pin(load_u8(self, memory, area));
        Box::pin(Reader::new(inner))
    }
}

impl LoadData<Bits16> for Registers {
    fn load_data(self, memory: Memory, area: Bits16) -> Loader {
        let inner = Box::pin(load_u16(self, memory, area));
        Box::pin(Reader::new(inner))
    }
}

impl LoadData<u16> for Registers {
    fn load_data(self, memory: Memory, area: u16) -> Loader {
        let data = self.borrow().get(Bits16::SP);
        let inner = Box::pin(memory.set::<u16>(area, data));
        Box::pin(Reader::new(inner))
    }
}
