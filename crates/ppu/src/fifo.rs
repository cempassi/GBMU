use shared::Error;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Fifo {
    queue: VecDeque<u8>,
}

impl<'push, 'fetch> Fifo {
    pub fn new() -> Self {
        let queue = VecDeque::with_capacity(16);
        Self { queue }
    }

    pub fn try_push(&mut self, data: &[u8; 8]) -> Result<(), Error> {
        let len = self.queue.len();
        if len <= 8 {
            // println!("[FETHCER] Pushed in the fifo: len {}", len);
            for i in data.iter() {
                self.queue.push_back(*i)
            }
            Ok(())
        } else {
            //println!("[FETCHER] Could not push to fifo: len {}", len);
            Err(Error::FifoNotReady)
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear()
    }

    pub fn try_pop(&mut self) -> Option<u8> {
        let len = self.queue.len();
        //println!("[FIFO] State: {:?}", self.queue);
        if len > 8 {
            self.queue.pop_front()
        } else {
            None
        }
    }
}
