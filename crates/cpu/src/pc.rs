use memory::Memory;
use shared::Error;
use std::primitive::{u16, u8};

//If successfull, the next function returns the byte at PC address and advances PC by one

pub type Pc = u16;

impl NextPc<u16> for Pc {
    fn next(&mut self, memory: Memory) -> Result<u16, Error> {
        let left: u8 = memory.borrow().get(*self)?;
        *self = self.wrapping_add(1);
        let right: u8 = memory.borrow().get(*self)?;
        *self = self.wrapping_add(1);
        Ok((left as u16) << 8 | right as u16)
    }
}

impl NextPc<u8> for Pc {
    fn next(&mut self, memory: Memory) -> Result<u8, Error> {
        let byte: u8 = memory.borrow().get(*self)?;
        *self = self.wrapping_add(1);
        Ok(byte)
    }
}

pub(crate) trait NextPc<T> {
    fn next(&mut self, memory: Memory) -> Result<T, Error>;
}
