use super::state::Cycle;
use crate::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[allow(dead_code)]
pub struct Getter<T> {
    cycle: Cycle,
    memory: Memory,
    address: u16,
    data: T,
}

impl<T> Getter<T> {
    pub fn new(memory: Memory, address: u16, data: T) -> Self {
        let cycle = Cycle::Cpu(0);
        Self {
            memory,
            address,
            cycle,
            data,
        }
    }
}

impl Future for Getter<u8> {
    type Output = Result<u8, Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => Poll::Ready(self.memory.borrow().get_u8(self.address)),
            Cycle::Cpu(ref mut ticks) => {
                *ticks += 1;
                if *ticks >= 4 {
                    self.cycle = Cycle::Finished;
                }
                Poll::Pending
            }
        }
    }
}

impl Future for Getter<u16> {
    type Output = Result<u16, Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => Poll::Ready(self.memory.borrow().get_u16(self.address)),
            Cycle::Cpu(ref mut ticks) => {
                *ticks += 1;
                if *ticks >= 8 {
                    self.cycle = Cycle::Finished;
                }
                Poll::Pending
            }
        }
    }
}

pub struct Setter<T> {
    memory: Memory,
    cycle: Cycle,
    address: u16,
    data: T,
}

impl<T> Setter<T> {
    pub fn new(memory: Memory, address: u16, data: T) -> Self {
        let cycle = Cycle::Cpu(0);
        Self {
            cycle,
            memory,
            address,
            data,
        }
    }
}

impl Future for Setter<u8> {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => {
                let address = self.address;
                let data = self.data;
                Poll::Ready(self.memory.borrow_mut().set_u8(address, data))
            }
            Cycle::Cpu(ref mut ticks) => {
                *ticks += 1;
                if *ticks >= 4 {
                    self.cycle = Cycle::Finished;
                }
                Poll::Pending
            }
        }
    }
}

impl Future for Setter<u16> {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => {
                let address = self.address;
                let data = self.data;
                Poll::Ready(self.memory.borrow_mut().set_u16(address, data))
            }
            Cycle::Cpu(ref mut ticks) => {
                *ticks += 1;
                if *ticks >= 8 {
                    self.cycle = Cycle::Finished;
                }
                Poll::Pending
            }
        }
    }
}
