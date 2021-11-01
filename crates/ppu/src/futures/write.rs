use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Ppu;

pub struct Writer<'write> {
    ticks: u8,
    ppu: &'write Ppu,
}

impl<'write> Writer<'write> {
    pub fn new(ppu: &'write Ppu) -> Self {
        Self { ticks: 0, ppu }
    }
}

impl<'write> Future for Writer<'write> {
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.ticks += 1;
        let mut ppu = self.ppu.borrow_mut();
        match ppu.write() {
            Some(_) => Poll::Ready(self.ticks),
            None => Poll::Pending,
        }
    }
}
