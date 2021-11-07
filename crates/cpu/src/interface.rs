use memory::Memory;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Cpu(Rc<RefCell<super::cpu::Cpu>>);

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            0: Rc::new(RefCell::new(super::cpu::Cpu::new(memory))),
        }
    }
}

impl Deref for Cpu {
    type Target = Rc<RefCell<super::cpu::Cpu>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cpu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait Access {
    fn memory(&self) -> Memory;
}

impl Access for Cpu {
    fn memory(&self) -> Memory {
        self.0.borrow().get_memory()
    }
}
