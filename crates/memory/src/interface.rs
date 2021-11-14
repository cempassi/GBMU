use crate::bus::MemoryBus;
use crate::mbc::MbcBus as Mbc;
use crate::memory;
use std::cell::RefCell;
use std::rc::Rc;

pub type Memory = Rc<RefCell<memory::Memory>>;

pub type Bus = Rc<RefCell<Box<dyn MemoryBus>>>;

pub type Rom = Rc<RefCell<Box<dyn Mbc>>>;
