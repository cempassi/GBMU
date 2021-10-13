use crate::{Reader, Registers};
use memory::Async;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

pub(crate) trait NextPc<T> {
    fn next_pc(self, memory: Memory) -> Pin<Box<dyn Future<Output = Result<T, Error>>>>;
}

impl NextPc<u8> for Registers {
    fn next_pc(self, memory: Memory) -> Pin<Box<dyn Future<Output = Result<u8, Error>>>> {
        let inner = Box::pin(next(self, memory));
        Box::pin(Reader::new(inner))
    }
}

impl NextPc<u16> for Registers {
    fn next_pc(self, memory: Memory) -> Pin<Box<dyn Future<Output = Result<u16, Error>>>> {
        let inner = Box::pin(next_16(self, memory));
        Box::pin(Reader::new(inner))
    }
}

async fn next(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let pc = registers.borrow().pc;
    let data = memory.get::<u8>(pc).await?;
    registers.borrow_mut().pc = pc.wrapping_add(1);
    Ok(data)
}

async fn next_16(registers: Registers, memory: Memory) -> Result<u16, Error> {
    let pc = registers.borrow().pc;
    let data = memory.get::<u16>(pc).await?;
    registers.borrow_mut().pc = pc.wrapping_add(2);
    Ok(data)
}
