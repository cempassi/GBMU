use crate::sprite::Sprite;
use crate::transfert::{Pixel, Pixels};
use shared::Error;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Fifo {
    discard: u8,
    queue: VecDeque<Pixel>,
    sprites: Vec<Sprite>,
}

impl<'push, 'fetch> Fifo {
    pub fn new() -> Self {
        let discard = 0;
        let queue = VecDeque::with_capacity(16);
        let sprites = Vec::new();
        Self {
            queue,
            discard,
            sprites,
        }
    }

    pub fn try_push(&mut self, data: &Pixels) -> Result<(), Error> {
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

    pub fn push_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite)
    }

    pub fn discard(&mut self, discard: u8) {
        self.discard = discard;
    }

    pub fn clear(&mut self) {
        self.queue.clear();
        self.sprites.clear();
    }

    pub fn is_sprite(&self, x: u8) -> bool {
        self.sprites.iter().any(|sprite| sprite.x == x)
    }

    pub fn try_pop(&mut self) -> Option<Pixel> {
        let len = self.queue.len();
        if len > 8 {
            // println!("[FIFO] Popped pixel from fifo: len {}", len);
            let pixel = self.queue.pop_front();
            match self.discard == 0 {
                true => pixel,
                false => {
                    //println!("[FIFO] Scrolling, discarded pixel");
                    self.discard -= 1;
                    None
                }
            }
        } else {
            //println!("[FIFO] Could not pop from fifo: len {}", len);
            None
        }
    }
}
