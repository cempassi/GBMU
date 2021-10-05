use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::cycle::Cycle;
use crate::interface::{Registers, Vram};

#[allow(dead_code)]
pub struct Oam {
    cycle: Cycle,
    registers: Registers,
    vram: Vram,
}

impl Oam {
    pub fn search(registers: Registers, vram: Vram) -> Self {
        let cycle = Cycle::Ppu(0);
        Self {
            cycle,
            vram,
            registers,
        }
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
                if *ticks == 40 {
                    println!("OAM Search finished: {} cycles", 40);
                    self.cycle = Cycle::Finished;
                }
                Poll::Pending
            }
        }
    }
}
