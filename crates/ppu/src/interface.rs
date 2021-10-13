use crate::registers;
use std::cell::RefCell;
use std::rc::Rc;

pub type Vram = Rc<RefCell<Vec<u8>>>;
pub type Registers = Rc<RefCell<registers::Registers>>;
