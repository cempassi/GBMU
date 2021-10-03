use super::state::Cycle;
use crate::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Getter {
    cycle: Cycle,
    memory: Memory,
    address: u16,
}

impl Getter {
    pub fn new(memory: Memory, address: u16) -> Self {
        let cycle = Cycle::Cpu(0);
        Self {
            memory,
            address,
            cycle,
        }
    }
}

impl Future for Getter {
    type Output = Result<u8, Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => Poll::Ready(self.memory.borrow().get(self.address)),
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

pub struct Setter {
    memory: Memory,
    cycle: Cycle,
    address: u16,
    data: u8,
}

impl Setter {
    pub fn new(memory: Memory, address: u16, data: u8) -> Self {
        let cycle = Cycle::Cpu(0);
        Self {
            cycle,
            memory,
            address,
            data,
        }
    }
}

impl Future for Setter {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => {
                let address = self.address;
                let data = self.data;
                Poll::Ready(self.memory.borrow_mut().set(address, data))
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
