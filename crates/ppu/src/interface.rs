use std::cell::RefCell;
use std::rc::Rc;

pub type Ppu = Rc<RefCell<crate::ppu::Ppu>>;
