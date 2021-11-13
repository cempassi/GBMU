use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Ppu;
use shared::Error;

pub struct Fetch<'fetch> {
    ticks: u8,
    ppu: &'fetch Ppu,
    address: u16,
}

impl<'fetch> Fetch<'fetch> {
    pub fn new(ppu: &'fetch Ppu, address: u16) -> Self {
        Self {
            ticks: 0,
            ppu,
            address,
        }
    }
}

impl<'fetch> Future for Fetch<'fetch> {
    type Output = Result<(u8, u8), Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ticks == 2 {
            // Maybe implement check mode Here?
            if self.ppu.borrow().vram_lock {
                match self.ppu.borrow().get_vram(self.address) {
                    Ok(data) => Poll::Ready(Ok((data, self.ticks))),
                    Err(err) => Poll::Ready(Err(err)),
                }
            } else {
                Poll::Ready(Ok((0xFF, 2)))
            }
        } else {
            self.ticks += 1;
            Poll::Pending
        }
    }
}
