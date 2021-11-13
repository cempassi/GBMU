use crate::blanks::Blank;
use crate::oam::Oam;
use crate::registers as lcd;
use crate::registers::Mode;
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
    if !ppu.borrow().registers.control.lcd_enabled {
        println!("[PPU] Ppu disabled");
        Ok(Finished::Nope)
    } else if ppu.borrow_mut().registers.is_lower(lcd::Field::Ly, 144) {
        let mut ticks = Oam::search(ppu.clone()).await;
        ticks += Pixel::transfert(ppu.clone()).start().await?;
        ticks += Blank::new(ppu.clone(), Mode::Hblank(204)).await;
        ppu.borrow_mut().registers.increase(lcd::Field::Ly);
        Ok(Finished::Line(ticks))
    } else {
        ppu.borrow().raise_vblank();
        ppu.borrow_mut().registers.mode.update(Mode::Vblank);
        Blank::new(ppu.clone(), Mode::Vblank).await;
        ppu.borrow_mut().registers.clear(lcd::Field::Ly);
        Ok(Finished::Frame)
    }
}
