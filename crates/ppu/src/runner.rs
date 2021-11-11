use crate::blanks::{Blank, HBLANK, VBLANK};
use crate::oam::Oam;
use crate::registers as lcd;
use crate::transfert::Pixel;
use crate::Ppu;
use shared::{Error, Finished, Output, Run};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Runner<T> {
    inner: Pin<Box<dyn Future<Output = T>>>,
}

impl<T> Runner<T> {
    pub fn new(inner: Pin<Box<dyn Future<Output = T>>>) -> Self {
        Self { inner }
    }
}

impl<T> Future for Runner<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(context)
    }
}

impl Run for Ppu {
    fn run(self) -> Output {
        let inner = Box::pin(run(self));
        Box::pin(Runner::new(inner))
    }
}

async fn run(ppu: Ppu) -> Result<Finished, Error> {
    if ppu.borrow_mut().registers.is_lower(lcd::Field::Ly, 144) {
        ppu.borrow_mut().x = 0;
        Oam::search(ppu.clone()).await;
        let cycles = Pixel::transfert(ppu.clone()).start().await?;
        println!("[FETCHER] fetcher ticks: {}", cycles);
        Blank::new(HBLANK).await;
        ppu.borrow_mut().registers.increase(lcd::Field::Ly);
        Ok(Finished::Line(42))
    } else {
        ppu.borrow().raise_vblank();
        Blank::new(VBLANK).await;
        ppu.borrow_mut().registers.clear(lcd::Field::Ly);
        ppu.borrow_mut().frame_ready = true;
        Ok(Finished::Frame(42))
    }
}
