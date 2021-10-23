use crate::Registers;
use memory::Async;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Next<T> =  Pin<Box<dyn Future<Output = Result<(T, u8), Error>>>>;

pub(crate) trait NextPc<T> {
    fn next_pc(self, memory: Memory) -> Next<T>;
}

impl NextPc<u8> for Registers {
    fn next_pc(self, memory: Memory) -> Next<u8>{
        Box::pin(next(self, memory))
    }
}

impl NextPc<u16> for Registers {
    fn next_pc(self, memory: Memory) -> Next<u16>{
        Box::pin(next_16(self, memory))
    }
}

async fn next(registers: Registers, memory: Memory) -> Result<(u8, u8), Error> {
    let pc = registers.borrow().pc;
    let data = memory.get::<u8>(pc).await?;
    registers.borrow_mut().pc = pc.wrapping_add(1);
    Ok(data)
}

async fn next_16(registers: Registers, memory: Memory) -> Result<(u16, u8), Error> {
    let pc = registers.borrow().pc;
    let data = memory.get::<u16>(pc).await?;
    registers.borrow_mut().pc = pc.wrapping_add(2);
    Ok(data)
}
