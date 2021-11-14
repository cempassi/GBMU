use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;

use crate::Ppu;

pub struct Push<'push> {
    ticks: u8,
    ppu: &'push crate::Ppu,
    data: [u8; 8],
}

impl<'push, 'fetch> Push<'push> {
    pub fn new(ppu: &'push Ppu, data: [u8; 8]) -> Self {
        Self {
            ticks: 0,
            ppu,
            data,
        }
    }
}

impl<'push> Future for Push<'push> {
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.ticks += 1;
        match self.ppu.borrow_mut().fifo.try_push(&self.data) {
            Ok(_) => Poll::Ready(self.ticks),
            Err(_) => Poll::Pending,
        }
    }
}
