use shared::Interrupts;
use std::cell::RefCell;
use std::rc::Rc;

pub type Ppu = Rc<RefCell<crate::ppu::Ppu>>;

pub trait New {
    fn new(interrupts: Interrupts) -> Self;
}

impl New for Ppu {
    fn new(interrupts: Interrupts) -> Self {
        Rc::new(RefCell::new(super::ppu::Ppu::from(interrupts)))
    }
}
