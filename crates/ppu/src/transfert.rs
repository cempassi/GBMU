use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use memory::Memory;

use crate::cycle::Cycle;

pub struct Pixel {
    cycle: Cycle,
    //memory: Memory
}

impl Pixel {
    pub fn transfert(_memory: Memory) -> Self {
        let cycle = Cycle::Ppu(0);
        Self {
            cycle,
            //memory
        }
    }
}

impl Future for Pixel {
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => Poll::Ready(42),
            Cycle::Ppu(ref mut ticks) => {
                *ticks += 1;
                if *ticks == 160 {
                    println!("Pixel transfert finished: {} cycles", *ticks);
                    self.cycle = Cycle::Finished;
                }
                Poll::Pending
            }
        }
    }
}
