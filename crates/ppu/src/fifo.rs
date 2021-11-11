use shared::Error;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Fifo {
    xscroll: u8,
    queue: VecDeque<u8>,
}

impl<'push, 'fetch> Fifo {
    pub fn new() -> Self {
        let xscroll = 0;
        let queue = VecDeque::with_capacity(16);
        Self { queue, xscroll }
    }

    pub fn try_push(&mut self, data: &[u8]) -> Result<(), Error> {
        let len = self.queue.len();
        if len <= 8 {
            println!("[FETHCER] Pushed in the fifo: len {}", len);
            for i in data {
                self.queue.push_back(*i)
            }
            Ok(())
        } else {
            println!("[FETCHER] Could not push to fifo: len {}", len);
            Err(Error::FifoNotReady)
        }
    }

    pub fn scroll(&mut self, xscroll: u8) {
        self.xscroll = xscroll;
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn clear(&mut self) {
        self.queue.clear()
    }

    pub fn try_pop(&mut self) -> Option<u8> {
        if self.queue.len() > 8 {
            // println!("[FIFO] Popped pixel from fifo: len {}", len);
            let pixel = self.queue.pop_front();
            match self.xscroll == 0 {
                true => pixel,
                false => {
                    self.xscroll -= 1;
                    None
                }
            }
        } else {
            //println!("[FIFO] Could not pop from fifo: len {}", len);
            None
        }
    }
}
