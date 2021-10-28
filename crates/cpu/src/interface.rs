use crate::registers;
use memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

pub type Registers = Rc<RefCell<registers::Registers>>;

pub type Cpu = Rc<RefCell<super::cpu::Cpu>>;

pub trait Access {
    fn registers(&self) -> Registers;
    fn memory(&self) -> Memory;
}

impl Access for Cpu {
    fn registers(&self) -> Registers {
        self.borrow().get_registers()
    }

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
