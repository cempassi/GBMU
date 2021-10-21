use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::cycle::Cycle;

pub const VBLANK: u32 = 83776;
pub const HBLANK: u32 = 272;

pub struct Blank {
    cycle: Cycle,
    length: u32,
}

impl Future for Blank {
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let length = self.length;

        match self.cycle {
            Cycle::Finished => Poll::Ready(42),
            Cycle::Ppu(ref mut ticks) => {
                *ticks += 1;
                if *ticks as u32 == length - 1 {
                    println!("Blank Period finished: {} cycles", ticks);
                    self.cycle = Cycle::Finished;
                }
                Poll::Pending
            }
        }
    }
}

impl Blank {
    pub fn new(length: u32) -> Self {
        let cycle = Cycle::Ppu(0);
        Self { cycle, length }
    }
}
