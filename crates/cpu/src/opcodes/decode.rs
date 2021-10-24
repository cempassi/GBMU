use crate::Registers;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

pub type Decode = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

pub trait Decoder {
    fn decode(self, registers: Registers, memory: Memory) -> Decode;
}
