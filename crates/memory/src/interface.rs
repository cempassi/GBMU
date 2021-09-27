use super::rom::{Cartridge, Mbc0};
use super::wram;
use crate::bios;
use crate::memory;
use crate::state::State;
use shared::{traits::Bus, Error};
use std::cell::RefCell;
use std::convert::From;
use std::rc::Rc;

pub type Memory = Rc<RefCell<memory::Memory>>;

pub trait NewMemory {
    fn new(mbc: Cartridge, data: Vec<u8>) -> Self;
}

struct Data(State, Bios, Rom, Wram);

impl From<Data> for memory::Memory {
    fn from(data: Data) -> Self {
        Self {
            state: data.0,
            bios: data.1,
            rom: data.2,
            wram: data.3,
        }
    }
}

impl NewMemory for Memory {
    fn new(mbc: Cartridge, data: Vec<u8>) -> Self {
        let rom: Rom = Rc::new(RefCell::new(match mbc {
            Cartridge::Mbc0 => crate::rom::Mbc0::new(data),
            _ => unimplemented!(),
        }));
        let state = State::Bios;
        let bios = Bios::default();
        let wram = Wram::default();
        let memory: memory::Memory = Data(state, bios, rom, wram).into();

        Rc::new(RefCell::new(memory))
    }
}

pub type Wram = Rc<RefCell<wram::Wram>>;

pub type Rom = Rc<RefCell<dyn Bus<usize, Item = u8, Result = Result<(), Error>, Data = u8>>>;

pub trait RomDefault {
    fn default() -> Self;
}

impl RomDefault for Rom {
    fn default() -> Self {
        Rc::new(RefCell::new(Mbc0::default()))
    }
}

pub type Bios = Rc<RefCell<bios::Bios>>;
