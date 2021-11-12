use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::registers::Mode;
use crate::Ppu;

const OAM_PERIOD: u16 = 80; // 77-83 cycles, 80 average

#[allow(dead_code)]
pub struct Oam {
    ticks: u16,
    ppu: Ppu,
}

impl Oam {
    pub fn search(ppu: Ppu) -> Self {
        let ticks = 0;
        ppu.borrow_mut().registers.mode.update(Mode::Oam);
        Self { ticks, ppu }
    }
}

impl Future for Oam {
    type Output = u16;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.ticks += 1;
        if self.ticks == OAM_PERIOD {
            Poll::Ready(self.ticks)
        } else {
            Poll::Pending
        }
    }
}
