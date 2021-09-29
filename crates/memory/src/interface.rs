use super::wram;
use crate::bios;
use crate::mbc::Mbc;
use crate::memory;
use std::cell::RefCell;
use std::rc::Rc;

pub type Memory = Rc<RefCell<memory::Memory>>;

pub type Wram = Rc<RefCell<wram::Wram>>;

pub type Rom = Rc<RefCell<Box<dyn Mbc>>>;

pub type Bios = Rc<RefCell<bios::Bios>>;
