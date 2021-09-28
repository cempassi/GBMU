use std::fmt::Debug;

pub trait MemoryBus: Debug {
    fn get(&self, _: usize) -> u8;
    fn set(&mut self, _: usize, data: u8);
}
