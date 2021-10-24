use std::future::Future;
use std::pin::Pin;
use shared::Error;
use crate::Registers;
use memory::Memory;

pub type Decode = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

pub trait Decoder {
    fn decode(self, registers: Registers, memory: Memory) -> Decode;
}
