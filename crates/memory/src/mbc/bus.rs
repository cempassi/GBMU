use crate::MemoryBus;
use shared::Error;

pub trait MbcBus: std::fmt::Debug {
    fn set(&mut self, address: usize, data: u8) -> Result<(), Error>;
}

pub trait Mbc: MbcBus + MemoryBus {}
