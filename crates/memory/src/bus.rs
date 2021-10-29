use shared::Error;
use std::convert::AsRef;

pub trait MemoryBus: std::fmt::Debug + AsRef<Vec<u8>> {
    fn get(&self, _: usize) -> Result<u8, Error>;
    fn set(&mut self, _: usize, data: u8) -> Result<(), Error>;
}
