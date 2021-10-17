use crate::registers;
use std::cell::RefCell;
use std::rc::Rc;

pub type Registers = Rc<RefCell<registers::Registers>>;

pub trait New {
    fn new() -> Self;
}

impl New for Registers {
    fn new() -> Self {
        Rc::new(RefCell::new(registers::Registers::default()))
    }
}
