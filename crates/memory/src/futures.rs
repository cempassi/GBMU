use super::state::Cycle;
use crate::Memory;
use ppu::Ppu;
use shared::Interrupts;
use shared::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Getter<T> {
    cycle: Cycle,
    memory: Memory,
    interrupts: Interrupts,
    ppu: Ppu,
    address: u16,
    output: PhantomData<T>,
}

impl<T> Getter<T> {
    pub fn new(memory: Memory, address: u16) -> Self {
        let cycle = Cycle::Cpu(0);
        let output = PhantomData;
        let interrupts = memory.borrow().interrupts.get_raisable();
        let ppu = memory.borrow().get_ppu();
        Self {
            memory,
            address,
            interrupts,
            ppu,
            cycle,
            output,
        }
    }
}

impl Future for Getter<u8> {
    type Output = Result<(u8, u8), Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => match self.memory.borrow().get_u8(self.address) {
                Ok(data) => Poll::Ready(Ok((data, 4))),
                Err(err) => Poll::Ready(Err(err)),
            },
            Cycle::Cpu(ref mut ticks) => {
                *ticks += 1;
                if *ticks == 4 {
                    self.cycle = Cycle::Finished;
                }
                //self.ppu.borrow_mut().step(&self.interrupts);
                Poll::Pending
            }
        }
    }
}

impl Future for Getter<u16> {
    type Output = Result<(u16, u8), Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => match self.memory.borrow().get_u16(self.address) {
                Ok(data) => Poll::Ready(Ok((data, 8))),
                Err(err) => Poll::Ready(Err(err)),
            },
            Cycle::Cpu(ref mut ticks) => {
                *ticks += 1;
                if *ticks == 8 {
                    self.cycle = Cycle::Finished;
                }
                //self.ppu.borrow_mut().step(&self.interrupts);
                Poll::Pending
            }
        }
    }
}

pub struct Setter<T> {
    memory: Memory,
    ppu: Ppu,
    interrupts: Interrupts,
    cycle: Cycle,
    address: u16,
    data: T,
}

impl<T> Setter<T> {
    pub fn new(memory: Memory, address: u16, data: T) -> Self {
        let cycle = Cycle::Cpu(0);
        let interrupts = memory.borrow().interrupts.get_raisable();
        let ppu = memory.borrow().get_ppu();
        Self {
            cycle,
            ppu,
            interrupts,
            memory,
            address,
            data,
        }
    }
}

impl Future for Setter<u8> {
    type Output = Result<u8, Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => {
                let address = self.address;
                let data = self.data;
                match self.memory.borrow_mut().set_u8(address, data) {
                    Ok(_) => Poll::Ready(Ok(4)),
                    Err(err) => Poll::Ready(Err(err)),
                }
            }
            Cycle::Cpu(ref mut ticks) => {
                *ticks += 1;
                if *ticks == 3 {
                    self.cycle = Cycle::Finished;
                }
                //self.ppu.borrow_mut().step(&self.interrupts);
                Poll::Pending
            }
        }
    }
}

impl Future for Setter<u16> {
    type Output = Result<u8, Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => {
                let address = self.address;
                let data = self.data;
                match self.memory.borrow_mut().set_u16(address, data) {
                    Ok(_) => Poll::Ready(Ok(8)),
                    Err(err) => Poll::Ready(Err(err)),
                }
            }
            Cycle::Cpu(ref mut ticks) => {
                *ticks += 1;
                if *ticks == 7 {
                    self.cycle = Cycle::Finished;
                }
                //self.ppu.borrow_mut().step(&self.interrupts);
                Poll::Pending
            }
        }
    }
}
