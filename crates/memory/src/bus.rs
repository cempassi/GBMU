use std::convert::AsRef;

pub trait MemoryBus: std::fmt::Debug + AsRef<Vec<u8>> {
    fn get(&self, _: usize) -> u8;
    fn set(&mut self, _: usize, data: u8);
}
