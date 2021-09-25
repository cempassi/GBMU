pub mod cpu;
use std::cell::RefCell;
use std::rc::Rc;

pub use crate::cpu::area;
pub use crate::cpu::Cpu;

pub type Registers = Rc<RefCell<cpu::Registers>>;
