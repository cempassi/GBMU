use crate::registers;
use crate::vram;
use std::cell::RefCell;
use std::rc::Rc;

pub type Vram = Rc<RefCell<vram::Vram>>;
pub type Registers = Rc<RefCell<registers::Registers>>;
