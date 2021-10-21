use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::cycle::Cycle;
use crate::Ppu;

#[allow(dead_code)]
pub struct Oam {
    cycle: Cycle,
    ppu: Ppu,
}

impl Oam {
    pub fn search(ppu: Ppu) -> Self {
        let cycle = Cycle::Ppu(0);
        Self { cycle, ppu }
    }
}

impl Future for Oam {
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.cycle {
            Cycle::Finished => Poll::Ready(42),
            Cycle::Ppu(ref mut ticks) => {
                println!("Search OAM, currently at cycle {}", ticks);
                *ticks += 1;
                if *ticks == 39 {
                    println!("OAM Search finished: {} cycles", 40);
                    self.cycle = Cycle::Finished;
                }
                Poll::Pending
            }
        }
    }
}
