use crate::registers;
use std::cell::RefCell;
use std::rc::Rc;

pub type Registers = Rc<RefCell<registers::Registers>>;
