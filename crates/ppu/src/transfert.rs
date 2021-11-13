mod fetcher;
mod pixels;
use std::task::{Context, Poll};

use futures::Future;

use crate::{futures::Pop, registers::Mode};
use futures::pending;
use shared::Error;

use crate::Ppu;
use fetcher::Fetcher;

#[allow(dead_code)]
pub struct Pixel {
    ppu: Ppu,
}

impl Pixel {
    pub fn transfert(ppu: Ppu) -> Self {
        ppu.borrow_mut().registers.mode.update(Mode::Transfert);
        Self { ppu }
    }

    pub async fn start(self) -> Result<u16, Error> {
        // Remember: The fetcher works on a line basis
        // The future is created here, but the actual fetching is made
        // after
        let waker = shared::waker::create();
        let mut context = Context::from_waker(&waker);

        let fetcher = Fetcher::new(self.ppu.clone()).fetch();
        let mut fetching = Box::pin(fetcher);
        let mut pop = Box::pin(Pop::new(&self.ppu));

        self.ppu.borrow_mut().vram_lock = true;
        let cycles = loop {
            match fetching.as_mut().poll(&mut context) {
                Poll::Ready(_) => {
                    let fetcher = Fetcher::new(self.ppu.clone()).fetch();
                    fetching = Box::pin(fetcher);
                },
                Poll::Pending => (),
            }
            match pop.as_mut().poll(&mut context) {
                Poll::Ready(ticks) => {
                    break ticks;
                }
                Poll::Pending => (),
            }
            pending!();
        };
        self.ppu.borrow_mut().vram_lock = false;
        //println!("[FETCHER] fetcher ticks: {}", cycles);
        Ok(cycles)
    }
}
