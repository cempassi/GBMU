use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Ppu;

/// Pop pixels out of the fifo and add them to the current display buffer.
pub struct Pop<'write> {
    poped: usize,
    ticks: u16,
    ppu: &'write Ppu,
}

impl<'write> Pop<'write> {
    pub fn new(ppu: &'write Ppu) -> Self {
        let ticks = 0;
        let poped = 0;
        Self { poped, ticks, ppu }
    }
}

impl<'write> Future for Pop<'write> {
    type Output = u16;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut ppu = self.ppu.borrow_mut();
        self.ticks += 1;
        if let Some(pixel) = ppu.fifo.try_pop() {
            ppu.output(self.poped, pixel);
            self.poped += 1;
            //println!("[FIFO] Popped: {}", self.poped);
        }
        if self.poped == 160 {
            //println!("Exited from Pop");
            Poll::Ready(self.ticks)
        } else {
            Poll::Pending
        }
    }
}
