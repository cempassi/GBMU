use shared::Interrupts;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::futures;

#[derive(Debug, Clone)]
pub struct Ppu(Rc<RefCell<super::ppu::Ppu>>);

impl Deref for Ppu {
    type Target = Rc<RefCell<super::ppu::Ppu>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ppu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Ppu {
    pub fn new(interrupts: Interrupts, bios: bool) -> Self {
        Self {
            0: Rc::new(RefCell::new(super::ppu::Ppu::new(interrupts, bios))),
        }
    }
}

pub trait Push<'push> {
    fn push(&self, data: [u8; 8]) -> futures::Push;
}

impl<'push> Push<'push> for Ppu {
    fn push(&self, data: [u8; 8]) -> futures::Push {
        futures::Push::new(self, data)
    }
}
