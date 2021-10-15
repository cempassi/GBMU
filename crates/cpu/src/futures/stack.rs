use super::{GetAt, SetAt};
use crate::area::Bits16;
use crate::registers::Arithmetic;
use crate::{Reader, RegisterBus, Registers};
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Pusher = Pin<Box<dyn Future<Output = Result<(), Error>>>>;
type Popper = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

pub(crate) trait Push {
    fn push(self, memory: Memory, area: Bits16) -> Pusher;
}

async fn push(registers: Registers, memory: Memory, area: Bits16) -> Result<(), Error> {
    let data = registers.borrow().get(area);
    registers.clone().set_at(memory, Bits16::SP, data).await?;
    registers.borrow_mut().increase(Bits16::SP, 2);
    Ok(())
}

impl Push for Registers {
    fn push(self, memory: Memory, area: Bits16) -> Pusher {
        let inner = Box::pin(push(self, memory, area));
        Box::pin(Reader::new(inner))
    }
}

pub(crate) trait Pop {
    fn pop(self, memory: Memory, area: Bits16) -> Popper;
}

async fn pop(registers: Registers, memory: Memory, area: Bits16) -> Result<(), Error> {
    let data = registers.clone().get_at(memory, Bits16::SP).await?;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().decrease(Bits16::SP, 2);
    Ok(())
}

impl Pop for Registers {
    fn pop(self, memory: Memory, area: Bits16) -> Popper {
        let inner = Box::pin(pop(self, memory, area));
        Box::pin(Reader::new(inner))
    }
}
