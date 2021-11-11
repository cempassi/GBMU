use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Ppu;

/// Writes the output of the fifo to the current frame buffer.
/// The future is instant, but will poll until a Pop (and then a write)
/// succeeded. The future is never ending, so the responsability to ending
/// is elsewhere
pub struct Pop<'write> {
    poped: u8,
    ticks: u8,
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
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut ppu = self.ppu.borrow_mut();
        self.ticks += 1;
        if ppu.pop() {
            self.poped += 1;
        }
        if self.poped == 160 {
            Poll::Ready(self.ticks)
        } else {
            Poll::Pending
        }
    }
}
