use std::cell::RefCell;
use std::rc::Rc;

pub type Apu = Rc<RefCell<crate::apu::Apu>>;
