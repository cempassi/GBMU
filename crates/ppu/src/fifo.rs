use shared::Error;
use std::collections::VecDeque;

use crate::Ppu;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Pusher<'push> {
    ticks: u8,
    ppu: &'push crate::Ppu,
    data: Vec<u8>,
}

impl<'push, 'fetch> Pusher<'push> {
    pub fn new(ppu: &'push Ppu, data: Vec<u8>) -> Self {
        Self {
            ticks: 0,
            ppu,
            data,
        }
    }
}

impl<'push> Future for Pusher<'push> {
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.ticks += 1;
        match self.ppu.borrow_mut().fifo.try_push(&self.data) {
            Ok(_) => Poll::Ready(self.ticks),
            Err(_) => Poll::Pending,
        }
    }
}

#[derive(Debug)]
pub struct Fifo {
    queue: VecDeque<u8>,
}

impl<'push, 'fetch> Fifo {
    pub fn new() -> Self {
        let queue = VecDeque::with_capacity(16);
        Self { queue }
    }

    fn try_push(&mut self, data: &[u8]) -> Result<(), Error> {
        let len = self.queue.len();
        if len <= 8 {
            //println!("[FETHCER] Pushed in the fifo: len {}", len);
            for i in data {
                self.queue.push_back(*i)
            }
            Ok(())
        } else {
            //println!("[FETCHER] Could not push to fifo: len {}", len);
            Err(Error::FifoNotReady)
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        let len = self.queue.len();
        if len > 8 {
            // println!("[FIFO] Popped pixel from fifo: len {}", len);
            self.queue.pop_front()
        } else {
            //println!("[FIFO] Could not pop from fifo: len {}", len);
            None
        }
    }
}
