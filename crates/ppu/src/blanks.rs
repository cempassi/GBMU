use crate::registers::Mode;
use crate::Ppu;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// time in cycles for rendering vblank
const VBLANK: u16 = 4560; // 4,560 cycles for vblank

pub struct Blank {
    ppu: Ppu,
    mode: Mode,
    ticks: u16,
}

impl Future for Blank {
    type Output = u16;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.ticks += 1;
        match self.mode {
            Mode::Hblank(ticks) => {
                if self.ticks == ticks {
                    Poll::Ready(self.ticks)
                } else {
                    Poll::Pending
                }
            }
            Mode::Vblank => {
                if self.ticks % 456 == 0 {
                    self.ppu
                        .borrow_mut()
                        .registers
                        .coordinates
                        .increase(crate::Field::Ly);
                }
                if self.ticks == VBLANK {
                    Poll::Ready(self.ticks)
                } else {
                    Poll::Pending
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Blank {
    pub fn new(ppu: Ppu, mode: Mode) -> Self {
        let ticks = 0;
        ppu.borrow_mut().registers.mode.update(mode);
        Self { ppu, ticks, mode }
    }
}
