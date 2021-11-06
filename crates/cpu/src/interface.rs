use memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

pub type Cpu = Rc<RefCell<super::cpu::Cpu>>;

pub trait Access {
    fn memory(&self) -> Memory;
}

impl Access for Cpu {
    fn memory(&self) -> Memory {
        self.borrow().get_memory()
    }
}

pub trait Make {
    fn make(memory: Memory) -> Self;
}

impl Make for Cpu {
    fn make(memory: Memory) -> Self {
        Rc::new(RefCell::new(super::cpu::Cpu::new(memory)))
    }
}
