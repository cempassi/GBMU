use crate::blanks::{Blank, HBLANK, VBLANK};
use crate::oam::Oam;
use crate::registers::lcd;
use crate::transfert::Pixel;
use crate::Ppu;
use shared::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

type Output = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

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

pub trait Run {
    fn run(self) -> Output;
}

impl Run for Ppu {
    fn run(self) -> Output {
        let inner = Box::pin(run(self));
        Box::pin(Runner::new(inner))
    }
}

async fn run(ppu: Ppu) -> Result<u8, Error> {
    println!("Running the ppu!");
    if ppu.borrow_mut().is_lower(lcd::Field::Ly, 144) {
        Oam::search(ppu.clone()).await;
        Pixel::transfert(ppu.clone()).await;
        Blank::new(HBLANK).await;
        ppu.borrow_mut().increase(lcd::Field::Ly);
        let ly = ppu.borrow_mut().get_lcd().get_lcd(lcd::Field::Ly);
        println!("Finished a cycle, Ly: {}", ly);
        Ok(42)
    } else {
        ppu.borrow().raise_vblank();
        Blank::new(VBLANK).await;
        ppu.borrow_mut().clear(lcd::Field::Ly);
        Ok(42)
    }
}
