use crate::registers;
use std::cell::RefCell;
use std::rc::Rc;

pub type Registers = Rc<RefCell<registers::Registers>>;

pub trait NewRegisters {
    fn new() -> Self;
}

impl NewRegisters for Registers {
    fn new() -> Self {
        Rc::new(RefCell::new(registers::Registers::default()))
    }
}
