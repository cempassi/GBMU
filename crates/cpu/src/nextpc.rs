use crate::Registers;
use memory::Async;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub(crate) trait NextPc<T> {
    fn next_pc(self, memory: Memory) -> Pin<Box<dyn Future<Output = Result<T, Error>>>>;
}

impl NextPc<u8> for Registers {
    fn next_pc(self, memory: Memory) -> Pin<Box<dyn Future<Output = Result<u8, Error>>>> {
        let inner = Box::pin(next(self, memory));
        Box::pin(Reader::new(inner))
    }
}

pub struct Reader<T> {
    inner: Pin<Box<dyn Future<Output = Result<T, Error>>>>,
}

impl<T> Reader<T> {
    pub fn new(inner: Pin<Box<dyn Future<Output = Result<T, Error>>>>) -> Self {
        Self { inner }
    }
}

impl<T> Future for Reader<T> {
    type Output = Result<T, Error>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.as_mut().poll(context) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(data) => Poll::Ready(data),
        }
    }
}

async fn next(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let pc = registers.borrow().pc;
    let byte: u8 = <Memory as Async>::get(memory, pc).await?;
    registers.borrow_mut().pc = pc.wrapping_add(1);
    Ok(byte)
}

async fn next_16(registers: Registers, memory: Memory) -> Result<u16, Error> {
    let pc = registers.borrow().pc;
    let left: u8 = <Memory as Async>::get(memory.clone(), pc).await?;
    registers.borrow_mut().pc = pc.wrapping_add(1);
    let pc = registers.borrow().pc;
    let right: u8 = <Memory as Async>::get(memory, pc).await?;
    registers.borrow_mut().pc = pc.wrapping_add(1);
    Ok((left as u16) << 8 | right as u16)
}

impl NextPc<u16> for Registers {
    fn next_pc(self, memory: Memory) -> Pin<Box<dyn Future<Output = Result<u16, Error>>>> {
        let inner = Box::pin(next_16(self, memory));
        Box::pin(Reader::new(inner))
    }
}
