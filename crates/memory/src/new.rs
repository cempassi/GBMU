use super::mbc::{Cartridge, Mbc0, Mbc1, Mbc2, Mbc3, Mbc5};
use crate::state::State;
use crate::{memory, Bios, Memory, Rom, Wram};
use std::cell::RefCell;
use std::rc::Rc;

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
            Cartridge::Mbc0 => Mbc0::new(data),
            Cartridge::Mbc1 => Mbc1::new(data),
            Cartridge::Mbc2 => Mbc2::new(data),
            Cartridge::Mbc3 => Mbc3::new(data),
            Cartridge::Mbc5 => Mbc5::new(data),
            _ => unimplemented!(),
        }));
        let state = State::Bios;
        let bios = Bios::default();
        let wram = Wram::default();
        let memory: memory::Memory = Data(state, bios, rom, wram).into();

        Rc::new(RefCell::new(memory))
    }
}
